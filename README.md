
*Resumo*
	
#	Programando um jogo de adivinhação.
	
Este projeto foi uma maneira prática de iniciar o conhecimento nos novos conceitos do Rust: let, match, funções, o uso de engradados externos e muito mais. Agora irei avançar para aprimorar e dominar a linguagem de programação Rust-lang.

## Configurando o projeto

- Configuração do ambiente.
- Criação dos arquivos necessários.
- Configuração do genrenciador de *pacotes (Cargo)*.

### Condificação do projeto.

Vamos mergulhar no Rust trabalhando juntos em um projeto prático! Nesta parte iremos conhecer alguns conceitos comuns do Rust, mostrando como usá-los em um *programa real*. *Aprenderemos sobre let, match, métodos, funções associadas, caixas externas e muito mais!* Nas próximas partes, exploraremos essas ideias com mais detalhes. No momento, vamos praticar apenas os fundamentos.

Implementaremos um problema clássico de programação quando estamos conhecendo uma linguagem de programação: *um jogo de adivinhação.* Veja como funciona: o programa irá gerar um número inteiro aleatório entre 1 e 100. Em seguida, solicitará que o jogador digite um palpite. Depois que um palpite é inserido, o programa indicará se o palpite é muito baixo ou muito alto. Se o palpite estiver correto, o jogo imprimirá uma mensagem de parabéns e sairá.