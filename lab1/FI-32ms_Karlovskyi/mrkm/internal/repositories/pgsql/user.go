package pgsql

import (
	"context"
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

func (r *UserRepository) FindBy(ctx context.Context, params map[string]interface{}) (account *entities.User, err error) {
	if err = r.conn.WithContext(ctx).Where(params).
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
