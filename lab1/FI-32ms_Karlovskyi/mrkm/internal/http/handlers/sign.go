package handlers

import (
	"encoding/hex"
	"github.com/gin-gonic/gin"
	"mrkm/internal/http"
	"mrkm/internal/services"
)

type RegisterRequest struct {
	Login    string `json:"login"`
	Password string `json:"password"`
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

type SignHandler struct {
	signService *services.SignService
}

func (h *SignHandler) Register(router *gin.RouterGroup) {
	sign := router.Group("ds")

	sign.POST("register", h.register)
	sign.POST("sign", h.sign)
	sign.POST("verify", h.verify)
}

func NewSignHandler(signService *services.SignService) *SignHandler {
	return &SignHandler{signService: signService}
}

func (h *SignHandler) register(ctx *gin.Context) {
	req := &RegisterRequest{}

	if err := ctx.ShouldBind(&req); err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	u, err := h.signService.Register(ctx, req.Login, req.Password)
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	http.OK(ctx, u, nil)
}

func (h *SignHandler) sign(ctx *gin.Context) {
	req := &SignRequest{}

	if err := ctx.ShouldBind(&req); err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	u, err := h.signService.Sign(ctx, []byte(req.Content), []byte(req.PrivateKey))
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	http.OK(ctx, hex.EncodeToString([]byte(u)), nil)
}

func (h *SignHandler) verify(ctx *gin.Context) {
	req := &VerifyRequest{}

	if err := ctx.ShouldBind(&req); err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	sign, err := hex.DecodeString(req.Signature)
	if err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	if err := h.signService.Verify(ctx, []byte(req.Content), sign, []byte(req.PublicKey)); err != nil {
		http.BadRequest(ctx, err, nil)

		return
	}

	http.OK(ctx, "success", nil)
}
