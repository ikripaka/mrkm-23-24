package http

import "github.com/gin-gonic/gin"

type Handler interface {
	Register(router *gin.RouterGroup)
}
