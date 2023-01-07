/*
Copyright © 2023 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

// addRepoCmd represents the addRepo command
var addRepoCmd = &cobra.Command{
	Use:   "add-repo",
	Short: "A brief description of your command",
	Long: `A longer description that spans multiple lines and likely contains examples
and usage of using your command. For example:

Cobra is a CLI library for Go that empowers applications.
This application is a tool to generate the needed files
to quickly create a Cobra application.`,
	Run: AddRepo,
}

func init() {
	rootCmd.AddCommand(addRepoCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// addRepoCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// addRepoCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

func AddRepo(cmd *cobra.Command, args []string) {
	fmt.Println(args[0])
}
