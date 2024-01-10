package handlers

import (
	"fmt"
	"github.com/gin-gonic/gin"
	"io"
	"mrkm/internal/http"
	"mrkm/internal/services"
)

type RegisterRequest struct {
	Nickname string `json:"nickname" form:"nickname"`
}

type SignRequest struct {
	Content    string `json:"content"`
	PrivateKey string `json:"private_key"`
}

type VerifyRequest struct {
	Content   string `json:"content"`
	PublicKey string `json:"public_key"`
	Signature string `json:"signature"`
}

type VerifyHandler struct {
	verifyService *services.VerifyService
}

func (h *VerifyHandler) Register(router *gin.RouterGroup) {
	sign := router.Group("ds")

	sign.POST("register", h.register)
	sign.POST("verify", h.verify)
}

func NewVerifyHandler(verifyService *services.VerifyService) *VerifyHandler {
	return &VerifyHandler{verifyService: verifyService}
}

func (h *VerifyHandler) register(ctx *gin.Context) {
	req := &RegisterRequest{}

	if err := ctx.ShouldBind(&req); err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	publicKey, err := contentFromFile(ctx, "public_key")
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	if err := h.verifyService.Register(ctx, req.Nickname, publicKey); err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	http.OK(ctx, "success", nil)
}

func (h *VerifyHandler) verify(ctx *gin.Context) {
	content, err := contentFromFile(ctx, "content")
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	publicKey, err := contentFromFile(ctx, "public_key")
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	signature, err := contentFromFile(ctx, "signature")
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	user, ok, err := h.verifyService.Verify(ctx, content, signature, publicKey)
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	ans := "success"

	if ok {
		ans += fmt.Sprintf(": username is %v", user.Nickname)
	} else {
		ans += " but user not registered"
	}

	http.OK(ctx, ans, nil)
}

func contentFromFile(ctx *gin.Context, key string) ([]byte, error) {
	formFile, err := ctx.FormFile(key)
	if err != nil {
		return nil, err
	}

	file, err := formFile.Open()
	if err != nil {
		return nil, err
	}

	defer file.Close()

	return io.ReadAll(file)
}
