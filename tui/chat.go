package tui

import (
	"fmt"
	"os"

	tea "github.com/charmbracelet/bubbletea"
)

type model struct {
	messages []string
	input    string
}

func initialModel() model {
	return model{
		messages: []string{},
		input:    "",
	}
}

func (m model) Init() tea.Cmd {
	return nil
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyMsg:
		switch msg.Type {
		case tea.KeyCtrlC, tea.KeyEsc:
			return m, tea.Quit
		case tea.KeyEnter:
			if m.input != "" {
				m.messages = append(m.messages, "You: "+m.input)
				// Echo back for now
				m.messages = append(m.messages, "Alfred: "+m.input)
				m.input = ""
			}
		default:
			if msg.Type == tea.KeySpace {
				m.input += " "
			} else if msg.Type == tea.KeyBackspace {
				if len(m.input) > 0 {
					m.input = m.input[:len(m.input)-1]
				}
			} else if len(msg.Runes) > 0 {
				m.input += string(msg.Runes)
			}
		}
	}
	return m, nil
}

func (m model) View() string {
	s := "Alfred CLI Chat\n\n"

	for _, msg := range m.messages {
		s += msg + "\n"
	}

	s += "\n> " + m.input
	s += "\n\nPress Esc or Ctrl+C to quit."
	return s
}

func Start() {
	p := tea.NewProgram(initialModel())
	if _, err := p.Run(); err != nil {
		fmt.Printf("Alas, there's been an error: %v", err)
		os.Exit(1)
	}
}
