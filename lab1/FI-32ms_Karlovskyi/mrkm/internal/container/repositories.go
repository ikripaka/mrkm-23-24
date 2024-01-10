package container

import (
	"github.com/sarulabs/di"
	"gorm.io/gorm"
	"mrkm/internal/constants"
	"mrkm/internal/repositories/pgsql"
)

func BuildRepositories() []di.Def {
	return []di.Def{
		{
			Name: constants.UserRepositoryName,
			Build: func(ctn di.Container) (interface{}, error) {
				conn := ctn.Get(constants.PgSQLName).(*gorm.DB)

				return pgsql.NewUserRepository(conn), nil
			},
		},
	}
}
