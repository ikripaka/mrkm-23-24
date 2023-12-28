package http

import (
	"github.com/samber/lo"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
)

type Response struct {
	Status  int         `json:"status"`
	Success bool        `json:"success"`
	Meta    interface{} `json:"meta"`
	Data    interface{} `json:"data"`
}

func new(status int, meta interface{}, data interface{}) *Response {
	success := false
	if status >= 200 && status <= 299 {
		success = true
	}

	response := &Response{
		Status:  status,
		Success: success,
		Meta:    meta,
		Data:    data,
	}

	if response.Data == nil {
		response.Data = http.StatusText(status)
	}

	if v, ok := data.(error); ok {
		response.Data = v.Error()
	}

	if v, ok := data.([]error); ok {
		response.Data = lo.Map(v, func(item error, index int) string {
			return item.Error()
		})
	}

	return response
}

func OK(ctx *gin.Context, data interface{}, meta interface{}) {
	r := new(http.StatusOK, meta, data)
	ctx.JSON(r.Status, r)
}

func BadRequest(ctx *gin.Context, data interface{}, meta interface{}) {
	zap.S().Warn(data)
	r := new(http.StatusBadRequest, meta, data)

	ctx.AbortWithStatusJSON(r.Status, r)
}

func Unauthorized(ctx *gin.Context, data interface{}, meta interface{}) {
	zap.S().Warn(data)
	r := new(http.StatusUnauthorized, meta, data)
	ctx.AbortWithStatusJSON(r.Status, r)
}

func Forbidden(ctx *gin.Context, data interface{}, meta interface{}) {
	zap.S().Warn(data)
	r := new(http.StatusForbidden, meta, data)
	ctx.AbortWithStatusJSON(r.Status, r)
}

func NotFound(ctx *gin.Context, data interface{}, meta interface{}) {
	zap.S().Warn(data)
	r := new(http.StatusNotFound, meta, data)
	ctx.AbortWithStatusJSON(r.Status, r)
}

func ServerError(ctx *gin.Context, data interface{}, meta interface{}) {
	zap.S().Error(data)
	r := new(http.StatusInternalServerError, meta, data)
	ctx.AbortWithStatusJSON(r.Status, r)
}

func Code(ctx *gin.Context, code int, data interface{}, meta interface{}) {
	zap.S().Warn(data)
	zap.S().Warn("code: " + strconv.Itoa(code))
	r := new(code, meta, data)
	ctx.AbortWithStatusJSON(r.Status, r)
}

func Conflict(ctx *gin.Context, data interface{}, meta interface{}) {
	zap.S().Error(data)
	r := new(http.StatusConflict, meta, data)
	ctx.AbortWithStatusJSON(r.Status, r)
}
