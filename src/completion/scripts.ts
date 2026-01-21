// Shell completion scripts for ElevenLabs CLI
// Generated from elevenlabs-completion.sh

export const bashCompletion = `#!/bin/bash
# ElevenLabs CLI Bash Completion
# 
# Usage: Add to your ~/.bashrc or ~/.bash_profile:
#   eval "$(elevenlabs completion bash)"

_elevenlabs_completion() {
  local cur prev cword
  COMPREPLY=()
  
  cur="\${COMP_WORDS[COMP_CWORD]}"
  prev="\${COMP_WORDS[COMP_CWORD-1]}"
  cword=\${COMP_CWORD}
  
  _add_completions() {
    local options="$1"
    local -a words=($options)
    for word in "\${words[@]}"; do
      [[ "$word" == "$cur"* ]] && COMPREPLY+=("$word")
    done
  }

  local commands="auth agents tools tests components completion -h --help -v --version"

  case $cword in
    1)
      _add_completions "$commands"
      ;;
    *)
      local auth_cmds="login logout whoami residency -h --help"
      local agents_cmds="init add list delete status push pull templates widget test -h --help"
      local agents_add_flags="--template --skip-upload"
      local agents_push_flags="--agent --dry-run"
      local agents_pull_flags="--agent --output-dir --dry-run --update --all --no-ui"
      local agents_templates_cmds="list show"
      local template_types="default minimal voice-only text-only customer-service assistant"
      
      local tools_cmds="add delete push pull -h --help"
      local tools_add_flags="--type --config-path"
      local tools_types="webhook client"
      
      local tests_cmds="add delete push pull -h --help"
      local components_cmds="add"
      local completion_shells="bash zsh"

      case \${COMP_WORDS[1]} in
        auth)
          _add_completions "$auth_cmds"
          ;;
        agents)
          if [[ $cword -eq 2 ]]; then
            _add_completions "$agents_cmds"
          else
            case \${COMP_WORDS[2]} in
              add)
                if [[ "$prev" == "--template" ]]; then
                  _add_completions "$template_types"
                else
                  _add_completions "$agents_add_flags"
                fi
                ;;
              templates)
                if [[ $cword -eq 3 ]]; then
                  _add_completions "$agents_templates_cmds"
                fi
                ;;
              push)
                if [[ "$prev" == "--agent" ]]; then
                  local agent_source=$(jq -r '.agents[].id' agents.json 2>/dev/null | sort -u)
                  _add_completions "$agent_source"
                else
                  _add_completions "$agents_push_flags"
                fi
                ;;
              pull)
                if [[ "$prev" == "--agent" ]]; then
                  local agent_source
                  if [[ " \${COMP_WORDS[@]} " =~ " --update " ]]; then
                    agent_source=$(jq -r '.agents[].id' agents.json 2>/dev/null | sort -u)
                  else
                    agent_source=$(elevenlabs agents pull --all --dry-run --no-ui 2>/dev/null | grep -o 'agent_[a-z0-9]*' | sort -u)
                  fi
                  _add_completions "$agent_source"
                else
                  _add_completions "$agents_pull_flags"
                fi
                ;;
              *)
                _add_completions "-h --help"
                ;;
            esac
          fi
          ;;
        tools)
          if [[ $cword -eq 2 ]]; then
            _add_completions "$tools_cmds"
          else
            case \${COMP_WORDS[2]} in
              add)
                if [[ "$prev" == "--type" ]]; then
                  _add_completions "$tools_types"
                else
                  _add_completions "$tools_add_flags"
                fi
                ;;
              *)
                _add_completions "-h --help"
                ;;
            esac
          fi
          ;;
        tests)
          if [[ $cword -eq 2 ]]; then
            _add_completions "$tests_cmds"
          fi
          ;;
        components)
          if [[ $cword -eq 2 ]]; then
            _add_completions "$components_cmds"
          fi
          ;;
        completion)
          if [[ $cword -eq 2 ]]; then
            _add_completions "$completion_shells"
          fi
          ;;
        *)
          ;;
      esac
      ;;
  esac

  return 0
}

complete -o bashdefault -o default -o nospace -F _elevenlabs_completion elevenlabs
`;

export const zshCompletion = `#compdef elevenlabs
# ElevenLabs CLI Zsh Completion
#
# Usage: Add to your ~/.zshrc:
#   eval "$(elevenlabs completion zsh)"

_elevenlabs_completion() {
  local -a auth_cmds agents_cmds agents_templates_cmds tools_cmds tests_cmds components_cmds
  local -a agents_add_opts agents_push_opts agents_pull_opts tools_add_opts
  
  auth_cmds=(
    "login:Authenticate with ElevenLabs using API key"
    "logout:Logout from ElevenLabs"
    "whoami:Check current login status"
    "residency:Set API residency/region"
  )
  
  agents_cmds=(
    "init:Initialize a new ElevenLabs project"
    "add:Create a new agent (agents add \\"Name\\" --template TYPE)"
    "list:List all agents in the project"
    "delete:Delete an agent from the project"
    "status:Check deployment status of agents"
    "push:Upload agents to ElevenLabs platform (--dry-run to preview)"
    "pull:Download agents from platform (--update to override local)"
    "templates:Manage agent templates"
    "widget:Generate HTML embed code for agent"
    "test:Test an agent"
  )
  
  agents_templates_cmds=(
    "list:Show available agent templates"
    "show:Display template configuration"
  )
  
  agents_add_opts=(
    "--template:Template type (default|minimal|voice-only|text-only|customer-service|assistant)"
    "--skip-upload:Create locally without uploading"
  )
  
  agents_push_opts=(
    "--agent:Specific agent ID to push"
    "--dry-run:Preview changes without uploading"
  )
  
  agents_pull_opts=(
    "--agent:Pull a specific agent by ID"
    "--output-dir:Output directory for configs (default: agent_configs)"
    "--dry-run:Preview changes without downloading"
    "--update:Update existing items only, skip new"
    "--all:Pull all (both new and existing)"
    "--no-ui:Disable interactive UI"
  )
  
  tools_cmds=(
    "add:Add a new tool (--type webhook|client --config-path FILE)"
    "delete:Delete a tool"
    "push:Upload tools to platform"
    "pull:Download tools from platform"
  )
  
  tools_add_opts=(
    "--type:Tool type (webhook|client)"
    "--config-path:Path to tool configuration file"
  )
  
  tests_cmds=(
    "add:Add a new test"
    "delete:Delete a test"
    "push:Upload tests to platform"
    "pull:Download tests from platform"
  )
  
  components_cmds=(
    "add:Import component from ElevenLabs UI registry"
  )

  case "$2" in
    auth)
      _describe "auth subcommand" auth_cmds
      ;;
    agents)
      if [[ $#COMP_WORDS -eq 3 ]]; then
        _describe "agents subcommand" agents_cmds
      else
        case "$3" in
          add)
            _describe "agents add option" agents_add_opts
            ;;
          templates)
            _describe "agents templates subcommand" agents_templates_cmds
            ;;
          push)
            if [[ "$COMP_WORDS[-2]" == "--agent" ]]; then
              local agent_list=($(jq -r '.agents[] | "\\(.id):\\(.config | split(\\"/\\") | .[-1] | sub(\\".json$\\"; \\"\\"))"' agents.json 2>/dev/null | sort -u))
              _describe "agent ID" agent_list
            else
              _describe "agents push option" agents_push_opts
            fi
            ;;
          pull)
            if [[ "$COMP_WORDS[-2]" == "--agent" ]]; then
              local agent_list
              if [[ " \${COMP_WORDS[@]} " =~ " --update " ]]; then
                agent_list=($(jq -r '.agents[] | "\\(.id):\\(.config | split(\\"/\\") | .[-1] | sub(\\".json$\\"; \\"\\"))"' agents.json 2>/dev/null | sort -u))
              else
                agent_list=($(elevenlabs agents pull --all --dry-run --no-ui 2>/dev/null | grep -o 'agent_[a-z0-9]*' | sort -u))
              fi
              _describe "agent ID" agent_list
            else
              _describe "agents pull option" agents_pull_opts
            fi
            ;;
        esac
      fi
      ;;
    tools)
      if [[ $#COMP_WORDS -eq 3 ]]; then
        _describe "tools subcommand" tools_cmds
      else
        case "$3" in
          add)
            _describe "tools add option" tools_add_opts
            ;;
        esac
      fi
      ;;
    tests)
      if [[ $#COMP_WORDS -eq 3 ]]; then
        _describe "tests subcommand" tests_cmds
      fi
      ;;
    components)
      if [[ $#COMP_WORDS -eq 3 ]]; then
        _describe "components subcommand" components_cmds
      fi
      ;;
    completion)
      local -a shells
      shells=("bash:Generate bash completion script" "zsh:Generate zsh completion script")
      _describe "shell" shells
      ;;
    *)
      local -a commands
      commands=(
        "auth:Authentication and configuration"
        "agents:Manage ElevenLabs agents and configurations"
        "tools:Manage agent tools and integrations"
        "tests:Manage agent tests"
        "components:Import components from ElevenLabs UI registry"
        "completion:Generate shell completion script"
      )
      _describe "elevenlabs command" commands
      ;;
  esac
}

compdef _elevenlabs_completion elevenlabs
`;
