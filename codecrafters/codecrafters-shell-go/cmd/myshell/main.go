package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"os"
	"os/exec"
	"slices"
	"strings"
)

func cmdType(c string) {
	builtinCmds := map[string]string{
		"type": "1",
		"echo": "1",
		"exit": "1",
		"pwd":  "1",
		"cd":   "1",
	}
	if builtinCmds[c] == "1" {
		fmt.Fprintln(os.Stdout, c+" is a shell builtin")
	} else {
		path, err := exec.LookPath(c)
		if err == nil {
			fmt.Fprintln(os.Stdout, c+" is "+path)
		} else {
			fmt.Fprintln(os.Stdout, c+": not found")
		}
	}
}

func execCommand(c []string, stdOut *os.File, stdErr *os.File) {

	cmd := exec.Command(c[0], c[1:]...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = stdOut
	cmd.Stderr = stdErr

	cmd.Run()
}

func errMsg(e error) {
	if e != nil {
		fmt.Fprintln(os.Stderr, e)
	}
}

func parseCmdLine(cmdLine string) ([]string, string, string, bool, bool) {
	cmdLine = strings.TrimSpace(cmdLine)
	modeAppend := false
	modeErrAppend := false
	stdOut := ""
	stdErr := ""

	firstPart, secondPart, found := strings.Cut(cmdLine, "2>")

	errFirstPart, errSecondPart, errFound := strings.Cut(secondPart, ">")
	if errFound {
		stdErr = strings.TrimSpace(errSecondPart)
		modeErrAppend = true
	} else {
		stdErr = strings.TrimSpace(errFirstPart)
	}

	firstPart, secondPart, _ = strings.Cut(firstPart, "1>")

	if secondPart == "" {
		firstPart, secondPart, _ = strings.Cut(firstPart, ">")
	}
	cmdLine = strings.TrimSpace(firstPart)

	firstPart, secondPart, found = strings.Cut(secondPart, ">")

	if found {
		stdOut = strings.TrimSpace(secondPart)
		modeAppend = true
	} else {
		stdOut = strings.TrimSpace(firstPart)
	}

	command := []string{}
	temp := []rune{}
	doubleQuote := false
	singleQuote := false
	doubleQuoteEscapeChars := []rune{'\\', '$', '\n', '"'}
	escape := false

	for i, r := range cmdLine {

		if escape {
			temp = append(temp, r)
			escape = false
			continue
		}

		if r == '\\' {
			if singleQuote {
				temp = append(temp, r)
			} else if !singleQuote && !doubleQuote {
				escape = true
			} else if doubleQuote {
				if slices.Contains(doubleQuoteEscapeChars, rune(cmdLine[i+1])) {
					escape = true
				} else {
					temp = append(temp, r)
				}
			}
			continue
		}

		if r == '"' {
			if !doubleQuote && !singleQuote {
				doubleQuote = true
			} else if singleQuote {
				temp = append(temp, r)
			} else if doubleQuote {
				if i < len(cmdLine)-1 {
					if cmdLine[i+1] == '"' || cmdLine[i-1] == '"' {
						continue
					} else if cmdLine[i+1] == ' ' {
						doubleQuote = false
						if len(temp) > 0 {
							command = append(command, string(temp))
							temp = nil
						}
					}
				} else {
					doubleQuote = false
					if len(temp) > 0 {
						command = append(command, string(temp))
						temp = nil
					}
				}

			}
			continue
		}

		if r == '\'' {
			if !doubleQuote && !singleQuote {
				singleQuote = true
			} else if doubleQuote {
				temp = append(temp, r)
			} else if singleQuote {
				if i < len(cmdLine)-1 {
					if cmdLine[i+1] == '\'' || cmdLine[i-1] == '\'' {
						continue
					}
				}
				singleQuote = false
				if len(temp) > 0 {
					command = append(command, string(temp))
					temp = nil
				}
			}
			continue
		}

		if r == ' ' {
			if doubleQuote || singleQuote {
				temp = append(temp, r)
			} else if !doubleQuote && !singleQuote {
				if len(temp) > 0 {
					command = append(command, string(temp))
					temp = nil
				}
			}
			continue
		}
		temp = append(temp, r)
	}
	if len(temp) > 0 {
		command = append(command, string(temp))
	}
	// fmt.Fprintf(os.Stdout, "\n%d: %v\n", len(command), command)
	return command, stdOut, stdErr, modeAppend, modeErrAppend
}

func main() {
	for true {
		fmt.Fprint(os.Stdout, "$ ")

		// Wait for user input
		input, err := bufio.NewReader(os.Stdin).ReadString('\n')
		errMsg(err)

		command, stdOutFile, stdErrFile, append, errAppend := parseCmdLine(input)

		stdOut := os.Stdout
		stdErr := os.Stderr
		fileMode := os.O_CREATE | os.O_WRONLY
		if append {
			fileMode = fileMode | os.O_APPEND
		}
		if stdOutFile != "" {
			stdOut, err = os.OpenFile(stdOutFile, fileMode, 0644)
			if err != nil {
				panic(err)
			}
			defer stdOut.Close()
		}
		errFileMode := os.O_CREATE | os.O_WRONLY
		if errAppend {
			errFileMode = errFileMode | os.O_APPEND
		}
		if stdErrFile != "" {
			stdErr, err = os.OpenFile(stdErrFile, errFileMode, 0644)
			if err != nil {
				panic(err)
			}
			defer stdErr.Close()
		}

		if err == io.EOF {
			os.Exit(0)
		}

		switch command[0] {
		case "exit":
			os.Exit(0)
		case "echo":
			fmt.Fprintln(stdOut, strings.Join(command[1:], " "))
			fmt.Fprintf(stdErr, "")
		case "type":
			cmdType(command[1])
		case "pwd":
			if cwd, err := os.Getwd(); err == nil {
				fmt.Fprintln(os.Stdout, cwd)
			}
		case "cd":
			if command[1] == "~" {
				command[1] = os.Getenv("HOME")
			}
			if err := os.Chdir(command[1]); err != nil {
				fmt.Fprintln(os.Stderr, command[0]+": "+command[1]+": No such file or directory")
			}
		default:
			_, err := exec.LookPath(command[0])
			if err != nil {
				errMsg(errors.New(command[0] + ": command not found"))
			} else {
				execCommand(command, stdOut, stdErr)
			}

		}

	}
}
