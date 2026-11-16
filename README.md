# 🌐 Visualizador de Grafos com Dijkstra (Visualizador_Dijkstra-me)

Um visualizador interativo de grafos implementado em **Rust** utilizando **eframe** e **egui**, com foco no estudo e na demonstração visual do algoritmo de **Dijkstra** (menor caminho) para a disciplina de Teoria dos Grafos.

---

## 📌 Sobre o Projeto

O projeto nasceu com o objetivo didático (e recreativo) de criar uma interface visual rica para manipular grafos e acompanhar o algoritmo de Dijkstra em tempo real.

Em vez de apenas ver listas de adjacência ou tabelas no terminal, o programa renderiza os vértices e arestas em um canvas interativo onde você pode arrastar nós livremente e configurar caminhos.

---

## ✨ Funcionalidades

- **Renderização Visual Interativa**: Vértices e arestas desenhados em tempo real com estilo escuro (*dark mode*).
- **Arrastar e Soltar (Drag & Drop)**: Arraste qualquer nó com o mouse para reorganizar a visualização do grafo na tela.
- **Seleção de Origem e Destino**:
  - 🟢 **Origem**: Nó destacado em verde com brilho sutil.
  - 🔴 **Destino**: Nó destacado em vermelho com brilho sutil.
  - 🔵 **Nós Comuns**: Nós destacados em azul.
- **Informações do Vértice**: Exibição da identificação do nó e do seu respectivo **grau** (*degree*).
- **Arestas Ponderadas com Badges**: Visualização das conexões com os respectivos pesos destacados no centro de cada aresta.
- **Reset de Layout**: Botão para restaurar os nós para as posições iniciais.
- **Estruturas Base para Dijkstra**: Estruturas de dados preparadas para expansão e cálculo de menor caminho.

---

## 🛠️ Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)** (Edição 2024)
- **[egui](https://github.com/emilk/egui) / [eframe](https://docs.rs/eframe/)** (v0.37.0) — GUI em modo imediato (*immediate mode*) rápida e nativa.
- **rand** (v0.10.3) — Utilitários de números aleatórios.

---

## 📂 Estrutura do Repositório

```text
.
├── Graph_Rs/
│   ├── Cargo.toml         # Configuração de dependências do Rust
│   └── src/
│       └── Graph.rs       # Código-fonte principal (estruturas de grafo e interface egui)
├── README.md              # Documentação do projeto
└── .gitignore
```

---

## 🚀 Como Executar

### Pré-requisitos
Certifique-se de ter o compilador do Rust e o gerenciador de pacotes Cargo instalados:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Passo a passo
1. Clone ou acesse a pasta do repositório:
   ```bash
   cd ~/Projetos/Trecs_rust/Visualizador_Dijkstra-me/Graph_Rs
   ```

2. Compile e execute o projeto:
   ```bash
   cargo run
   ```

---

## 🎮 Controles e Interação

- **Clique e arraste em um nó**: Move o vértice pela janela.
- **Barra Superior**:
  - Selecione a **Origem** clicando na letra correspondente.
  - Selecione o **Destino** clicando na letra correspondente.
  - Clique em **Resetar Layout** para redefinir as coordenadas iniciais dos nós.
