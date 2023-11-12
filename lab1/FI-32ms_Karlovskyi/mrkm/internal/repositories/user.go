package repositories

import (
	"context"
	"mrkm/internal/entities"
)

type UserRepository interface {
	FindBy(ctx context.Context, publicKey []byte) (account *entities.User, err error)
	Create(ctx context.Context, user *entities.User) (*entities.User, error)
}
