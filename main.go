/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package main

import (
	"fmt"
	"os"

	"github.com/spf13/viper"
)

func main() {
	exe, err := os.Executable()
	if err != nil {
		os.Exit(2)
	}
	exe = exe[:len(exe)-4]
	viper.AddConfigPath(exe)
	viper.SetConfigName("config")
	viper.SetConfigType("toml")
	viper.ReadInConfig()
	fmt.Println(viper.Get("target"))
	// cmd.Execute()
}
