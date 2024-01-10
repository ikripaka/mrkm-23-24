package container

import (
	"github.com/sarulabs/di"
	"mrkm/internal/constants"
	"mrkm/internal/repositories"
	"mrkm/internal/services"
)

func BuildServices() []di.Def {
	return []di.Def{
		{
			Name: constants.SignServiceName,
			Build: func(ctn di.Container) (interface{}, error) {

				return services.NewSignService(), nil
			},
		},
		{
			Name: constants.VerifyServiceName,
			Build: func(ctn di.Container) (interface{}, error) {
				userRepo := ctn.Get(constants.UserRepositoryName).(repositories.UserRepository)

				return services.NewVerifyService(userRepo), nil
			},
		},
	}
}
