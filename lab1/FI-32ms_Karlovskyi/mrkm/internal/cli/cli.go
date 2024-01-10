package cli

import (
	"context"
	"fmt"
	"github.com/spf13/cobra"
	"mrkm/internal/entities"
	"mrkm/internal/services"
	"os"
	"path/filepath"
)

type Manager struct {
	rootCmd *cobra.Command

	signService *services.SignService
}

func NewManager(signService *services.SignService) *Manager {
	cliManager := &Manager{
		signService: signService,
		rootCmd: &cobra.Command{
			Use:   "",
			Short: "",
		},
	}

	cliManager.rootCmd.AddCommand(
		&cobra.Command{
			Use:   "generate",
			Short: "generates public and private rsa keys",
			Run:   cliManager.generate,
		},
		&cobra.Command{
			Use:   "sign",
			Args:  cobra.ExactArgs(2),
			Short: "sign file and receive signature",
			Run:   cliManager.sign,
		})

	return cliManager
}

func (m *Manager) Run() error {
	if err := m.rootCmd.Execute(); err != nil {
		return err
	}

	return nil
}

func (m *Manager) generate(cmd *cobra.Command, args []string) {
	path := "."

	if len(args) >= 1 {
		path = args[0]
	}

	key, err := m.signService.Generate(context.Background())
	if err != nil {
		fmt.Println(err)

		return
	}

	pub, priv := entities.FormatKey(key)

	path, err = filepath.Abs(path)
	if err != nil {
		fmt.Println(err)

		return
	}

	pubPath := filepath.Join(path, "key.pub")
	privPath := filepath.Join(path, "key.priv")

	pubFile, err := os.OpenFile(pubPath, os.O_CREATE|os.O_RDWR|os.O_TRUNC, 0666)
	if err != nil {
		fmt.Println(err)

		return
	}

	defer pubFile.Close()

	privFile, err := os.OpenFile(privPath, os.O_CREATE|os.O_RDWR|os.O_TRUNC, 0666)
	if err != nil {
		fmt.Println(err)

		return
	}

	defer privFile.Close()

	pubFile.Write(pub)
	privFile.Write(priv)

}

func (m *Manager) sign(cmd *cobra.Command, args []string) {
	if len(args) < 2 {
		fmt.Println("no file specified")

		return
	}

	var err error

	tbsPath := args[0]
	tbsPath, err = filepath.Abs(tbsPath)
	if err != nil {
		fmt.Println(err)

		return
	}

	privPath := args[1]
	privPath, err = filepath.Abs(privPath)
	if err != nil {
		fmt.Println(err)

		return
	}

	tbs, err := os.ReadFile(tbsPath)
	if err != nil {
		fmt.Println(err)

		return
	}

	priv, err := os.ReadFile(privPath)
	if err != nil {
		fmt.Println(err)

		return
	}

	signature, err := m.signService.Sign(context.Background(), tbs, priv)
	if err != nil {
		fmt.Println(err)

		return
	}

	signFile, err := os.OpenFile(tbsPath+".sign", os.O_CREATE|os.O_RDWR|os.O_TRUNC, 0666)
	if err != nil {
		fmt.Println(err)

		return
	}

	defer signFile.Close()

	fmt.Println(signature)
	signFile.Write(signature)
}
