package repositories

import (
	"context"
	"mrkm/internal/entities"
)

type UserRepository interface {
	FindBy(ctx context.Context, params map[string]interface{}) (account *entities.User, err error)
	Create(ctx context.Context, user *entities.User) (*entities.User, error)
}
