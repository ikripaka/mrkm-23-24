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
				verifyService := ctn.Get(constants.VerifyServiceName).(*services.VerifyService)

				return handlers.NewVerifyHandler(verifyService), nil
			},
		},
	}
}
