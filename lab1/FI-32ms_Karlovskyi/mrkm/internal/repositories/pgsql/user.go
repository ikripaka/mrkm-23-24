package pgsql

import (
	"context"
	"encoding/hex"
	"errors"
	"gorm.io/gorm"
	"mrkm/internal/entities"
	"mrkm/internal/errs"
)

type UserRepository struct {
	conn *gorm.DB
}

func NewUserRepository(conn *gorm.DB) *UserRepository {
	return &UserRepository{
		conn: conn,
	}
}

func (r *UserRepository) FindBy(ctx context.Context, publicKey []byte) (account *entities.User, err error) {
	if err = r.conn.WithContext(ctx).Where("public_key = decode(?, 'hex')", hex.EncodeToString(publicKey)).
		First(&account).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, errs.ErrAccountDoesNotExists
		}

		return
	}

	return
}

func (r *UserRepository) Create(ctx context.Context, account *entities.User) (*entities.User, error) {
	if err := r.conn.WithContext(ctx).Create(&account).Error; err != nil {
		return nil, err
	}

	return account, nil
}
