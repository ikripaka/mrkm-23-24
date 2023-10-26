package container

import (
	"github.com/sarulabs/di"
	"mrkm/internal/constants"
	"mrkm/internal/http/handlers"
	"mrkm/internal/services"
)

func BuildHandlers() []di.Def {
	return []di.Def{
		{
			Name: constants.SignHandlerName,
			Build: func(ctn di.Container) (interface{}, error) {
				signService := ctn.Get(constants.SignServiceName).(*services.SignService)

				return handlers.NewSignHandler(signService), nil
			},
		},
	}
}
