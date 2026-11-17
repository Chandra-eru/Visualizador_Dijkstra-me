use std::cmp::Reverse;
use std::collections::BinaryHeap;
use eframe::egui;
//************************************************************************************
#[derive(Clone, Debug)]
struct Arest {
    peso: u32,
    dcamin: usize,
}
//************************************************************************************
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Vertic {
    arestu: Vec<Arest>,
    grau: u32,
    num_nome: u32,
    cha_nome: char,
}
//************************************************************************************
#[derive(Clone, Debug)]
struct Grap {
    verticu: Vec<Vertic>,
}
//************************************************************************************
// Estrutura com os resultados calculados pelo Algoritmo de Dijkstra
//************************************************************************************
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct ResultadoDijkstra {
    origem: usize,
    destino: usize,
    distancias: Vec<u32>,
    pais: Vec<Option<usize>>,
    caminho: Vec<usize>,                  // Sequência de nós de origem até destino
    arestas_caminho: Vec<(usize, usize)>, // Arestas específicas do menor caminho
    arestas_subgrafo: Vec<(usize, usize)>,// Todas as arestas exploradas na árvore de caminhos mínimos
    custo_total: Option<u32>,
}

// Algoritmo de Dijkstra
fn calcular_dijkstra(grafu: &Grap, origem: usize, destino: usize) -> ResultadoDijkstra {
    let n = grafu.verticu.len();
    if origem >= n {
        return ResultadoDijkstra {
            origem,
            destino,
            distancias: vec![],
            pais: vec![],
            caminho: vec![],
            arestas_caminho: vec![],
            arestas_subgrafo: vec![],
            custo_total: None,
        };
    }

    let mut distancias = vec![u32::MAX; n];
    let mut pais: Vec<Option<usize>> = vec![None; n];
    let mut fila: BinaryHeap<(Reverse<u32>, usize)> = BinaryHeap::new();

    distancias[origem] = 0;
    fila.push((Reverse(0), origem));

    while let Some((Reverse(d), u)) = fila.pop() {
        if d > distancias[u] {
            continue;
        }

        if let Some(vertice) = grafu.verticu.get(u) {
            for aresta in &vertice.arestu {
                let v = aresta.dcamin;
                if v >= n {
                    continue;
                }
                let novo_custo = d.saturating_add(aresta.peso);
                if novo_custo < distancias[v] {
                    distancias[v] = novo_custo;
                    pais[v] = Some(u);
                    fila.push((Reverse(novo_custo), v));
                }
            }
        }
    }

    // Coleta todas as arestas que compõem a árvore gerada pelo Dijkstra (subgrafo completo)
    let mut arestas_subgrafo = Vec::new();
    for (v, &p) in pais.iter().enumerate() {
        if let Some(u) = p {
            arestas_subgrafo.push((u, v));
        }
    }

    // Reconstrói o caminho específico até o destino
    let mut caminho = Vec::new();
    let mut arestas_caminho = Vec::new();
    let custo_total = if destino < n && distancias[destino] != u32::MAX {
        let mut curr = destino;
        caminho.push(curr);
        while curr != origem {
            if let Some(ant) = pais[curr] {
                arestas_caminho.push((ant, curr));
                curr = ant;
                caminho.push(curr);
            } else {
                break;
            }
        }
        caminho.reverse();
        arestas_caminho.reverse();
        Some(distancias[destino])
    } else if origem == destino && destino < n {
        caminho.push(origem);
        Some(0)
    } else {
        None
    };

    ResultadoDijkstra {
        origem,
        destino,
        distancias,
        pais,
        caminho,
        arestas_caminho,
        arestas_subgrafo,
        custo_total,
    }
}
//************************************************************************************
// Retorna o subgrafo construído pelo Dijkstra
#[allow(dead_code)]
fn subgrafo_dijk(grafu: &Grap, origem: usize, destino: usize) -> Grap {
    let res = calcular_dijkstra(grafu, origem, destino);
    let n = grafu.verticu.len();
    let mut sub_verticu = Vec::with_capacity(n);

    for vert in &grafu.verticu {
        sub_verticu.push(Vertic {
            arestu: Vec::new(),
            grau: 0,
            num_nome: vert.num_nome,
            cha_nome: vert.cha_nome,
        });
    }

    for &(u, v) in &res.arestas_subgrafo {
        if let Some(vert_orig) = grafu.verticu.get(u) {
            if let Some(aresta_orig) = vert_orig.arestu.iter().find(|a| a.dcamin == v) {
                sub_verticu[u].arestu.push(aresta_orig.clone());
                sub_verticu[u].grau += 1;
            }
        }
    }

    Grap { verticu: sub_verticu }
}

// Retorna o vetor com a sequência de índices do caminho mínimo
#[allow(dead_code)]
fn caminho_dijk(grafu: &Grap, origem: usize, destino: usize) -> Vec<usize> {
    let res = calcular_dijkstra(grafu, origem, destino);
    res.caminho
}
//************************************************************************************
// Interface Gráfica com egui / eframe
//************************************************************************************
struct Aplicacao {
    grafo: Grap,
    posicoes: Vec<egui::Pos2>,
    arrastando: Option<usize>,
    no_origem: usize,
    no_destino: usize,
    resultado_dijkstra: Option<ResultadoDijkstra>,
    mostrar_subgrafo_completo: bool,
}
//************************************************************************************
impl Aplicacao {
    fn new(grafo: Grap) -> Self {
        let n = grafo.verticu.len();
        let mut posicoes = vec![
            egui::pos2(160.0, 180.0), // A (0)
            egui::pos2(380.0, 110.0), // B (1)
            egui::pos2(600.0, 180.0), // C (2)
            egui::pos2(260.0, 400.0), // D (3)
            egui::pos2(520.0, 400.0), // E (4)
            egui::pos2( 80.0, 340.0), // F (5)
            egui::pos2(700.0, 340.0), // G (6)
            egui::pos2(380.0, 520.0), // H (7)
            egui::pos2(160.0,  50.0), // I (8)
            egui::pos2(680.0,  50.0), // J (9)
        ];

        while posicoes.len() < n {
            let idx = posicoes.len();
            posicoes.push(egui::pos2(150.0 + (idx as f32) * 80.0, 220.0));
        }

        Self {
            grafo,
            posicoes,
            arrastando: None,
            no_origem: 0,
            no_destino: 4.min(n.saturating_sub(1)),
            resultado_dijkstra: None,
            mostrar_subgrafo_completo: false,
        }
    }
}
//************************************************************************************
impl eframe::App for Aplicacao {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let n = self.grafo.verticu.len();

        // 1. Barra Superior: Controles
        ui.horizontal_wrapped(|ui| {
            ui.heading("🌐 Visualizador de Grafos");
            ui.separator();

            ui.label(egui::RichText::new("Origem:").strong());
            for (i, v) in self.grafo.verticu.iter().enumerate() {
                let selecionado = self.no_origem == i;
                if ui.selectable_label(selecionado, v.cha_nome.to_string()).clicked() {
                    self.no_origem = i;
                    // Recalcula se o Dijkstra já estiver ativo
                    if self.resultado_dijkstra.is_some() {
                        self.resultado_dijkstra = Some(calcular_dijkstra(&self.grafo, self.no_origem, self.no_destino));
                    }
                }
            }

            ui.separator();
            ui.label(egui::RichText::new("Destino:").strong());
            for (i, v) in self.grafo.verticu.iter().enumerate() {
                let selecionado = self.no_destino == i;
                if ui.selectable_label(selecionado, v.cha_nome.to_string()).clicked() {
                    self.no_destino = i;
                    // Recalcula se o Dijkstra já estiver ativo
                    if self.resultado_dijkstra.is_some() {
                        self.resultado_dijkstra = Some(calcular_dijkstra(&self.grafo, self.no_origem, self.no_destino));
                    }
                }
            }

            ui.separator();

            // Botão Principal: Calcular Caminho
            let btn_calc = egui::Button::new(
                egui::RichText::new("⚡ Calcular Caminho")
                    .color(egui::Color32::BLACK)
                    .strong(),
            )
            .fill(egui::Color32::from_rgb(46, 204, 113));

            if ui.add(btn_calc).clicked() {
                self.resultado_dijkstra = Some(calcular_dijkstra(&self.grafo, self.no_origem, self.no_destino));
            }

            if self.resultado_dijkstra.is_some() {
                if ui.button("✕ Limpar").clicked() {
                    self.resultado_dijkstra = None;
                }

                ui.checkbox(
                    &mut self.mostrar_subgrafo_completo,
                    "Árvore Dijkstra completa",
                );
            }

            ui.separator();
            if ui.button("↺ Resetar Nós").clicked() {
                self.posicoes = vec![
                    egui::pos2(160.0, 180.0), // A (0)
                    egui::pos2(380.0, 110.0), // B (1)
                    egui::pos2(600.0, 180.0), // C (2)
                    egui::pos2(260.0, 400.0), // D (3)
                    egui::pos2(520.0, 400.0), // E (4)
                    egui::pos2( 80.0, 340.0), // F (5)
                    egui::pos2(700.0, 340.0), // G (6)
                    egui::pos2(380.0, 520.0), // H (7)
                    egui::pos2(160.0,  50.0), // I (8)
                    egui::pos2(680.0,  50.0), // J (9)
                ];
            }
        });

        // 2. Banner de Status / Resultado
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if let Some(res) = &self.resultado_dijkstra {
                if let Some(custo) = res.custo_total {
                    let rota_str = res
                        .caminho
                        .iter()
                        .filter_map(|&idx| self.grafo.verticu.get(idx).map(|v| v.cha_nome.to_string()))
                        .collect::<Vec<_>>()
                        .join(" → ");

                    ui.label(
                        egui::RichText::new(format!("✓ Caminho Mínimo: {} | Custo Total: {}", rota_str, custo))
                            .strong()
                            .color(egui::Color32::from_rgb(46, 204, 113)),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("⚠ Não existe caminho alcançável entre a Origem e o Destino!")
                            .strong()
                            .color(egui::Color32::from_rgb(231, 76, 60)),
                    );
                }
            } else {
                ui.label(
                    egui::RichText::new("💡 Clique em \"Calcular Caminho\" para destacar o subgrafo mínimo em verde.")
                        .small()
                        .color(egui::Color32::from_rgb(170, 175, 190)),
                );
            }
        });

        ui.separator();

        // 3. Área do Canvas
        let area_disponivel = ui.available_size_before_wrap();
        let (rect, resposta) = ui.allocate_exact_size(area_disponivel, egui::Sense::click_and_drag());
        let painter = ui.painter_at(rect);

        // Fundo escuro elegante
        painter.rect(
            rect,
            12.0,
            egui::Color32::from_rgb(18, 20, 26),
            egui::Stroke::new(1.5, egui::Color32::from_rgb(45, 52, 70)),
            egui::StrokeKind::Inside,
        );

        let raio_no = 26.0;

        // Movimentação dos nós com o mouse
        if let Some(pos_mouse) = resposta.hover_pos() {
            if resposta.drag_started() {
                for (i, &p) in self.posicoes.iter().enumerate().take(n) {
                    if p.distance(pos_mouse) <= raio_no {
                        self.arrastando = Some(i);
                        break;
                    }
                }
            }

            if resposta.dragged() {
                if let Some(idx) = self.arrastando {
                    if idx < self.posicoes.len() {
                        let margem = raio_no + 8.0;
                        let novo_x = pos_mouse.x.clamp(rect.min.x + margem, rect.max.x - margem);
                        let novo_y = pos_mouse.y.clamp(rect.min.y + margem, rect.max.y - margem);
                        self.posicoes[idx] = egui::pos2(novo_x, novo_y);
                    }
                }
            }
        }

        if resposta.drag_stopped() {
            self.arrastando = None;
        }

        // Helpers de destaque
        let esta_no_caminho = |u: usize, v: usize, res: &Option<ResultadoDijkstra>| -> bool {
            if let Some(r) = res {
                r.arestas_caminho.iter().any(|&(a, b)| (a == u && b == v) || (a == v && b == u))
            } else {
                false
            }
        };

        let esta_no_subgrafo = |u: usize, v: usize, res: &Option<ResultadoDijkstra>| -> bool {
            if let Some(r) = res {
                r.arestas_subgrafo.iter().any(|&(a, b)| (a == u && b == v) || (a == v && b == u))
            } else {
                false
            }
        };

        // 4. Desenha as Arestas do Grafo
        for (origem, vertice) in self.grafo.verticu.iter().enumerate() {
            for aresta in &vertice.arestu {
                let destino = aresta.dcamin;
                if destino >= self.posicoes.len() || origem >= self.posicoes.len() {
                    continue;
                }

                // Evita desenhar duplicado arestas bidirecionais
                if origem > destino && vertice_tem_aresta(&self.grafo, destino, origem) {
                    continue;
                }

                let inicio = self.posicoes[origem];
                let fim = self.posicoes[destino];

                let no_caminho = esta_no_caminho(origem, destino, &self.resultado_dijkstra);
                let no_subgrafo = self.mostrar_subgrafo_completo && esta_no_subgrafo(origem, destino, &self.resultado_dijkstra);

                if no_caminho {
                    // Efeito de Brilho / Glow Neon Verde
                    painter.line_segment(
                        [inicio, fim],
                        egui::Stroke::new(8.0, egui::Color32::from_rgba_premultiplied(46, 204, 113, 60)),
                    );
                    // Linha Verde Principal do Caminho
                    painter.line_segment(
                        [inicio, fim],
                        egui::Stroke::new(4.5, egui::Color32::from_rgb(46, 204, 113)),
                    );
                } else if no_subgrafo {
                    // Linha Verde Suave para outras arestas da árvore Dijkstra
                    painter.line_segment(
                        [inicio, fim],
                        egui::Stroke::new(3.0, egui::Color32::from_rgb(39, 174, 96)),
                    );
                } else {
                    // Linha Normal Cinza
                    painter.line_segment(
                        [inicio, fim],
                        egui::Stroke::new(2.2, egui::Color32::from_rgb(110, 120, 145)),
                    );
                }

                // Badge de Peso no meio da aresta
                let meio = egui::pos2((inicio.x + fim.x) * 0.5, (inicio.y + fim.y) * 0.5);
                let badge_size = egui::vec2(24.0, 20.0);
                let badge_rect = egui::Rect::from_center_size(meio, badge_size);

                let (cor_fundo_badge, cor_borda_badge, cor_texto_badge) = if no_caminho {
                    (
                        egui::Color32::from_rgb(20, 50, 30),
                        egui::Color32::from_rgb(46, 204, 113),
                        egui::Color32::from_rgb(255, 255, 255),
                    )
                } else {
                    (
                        egui::Color32::from_rgb(30, 34, 45),
                        egui::Color32::from_rgb(70, 80, 105),
                        egui::Color32::from_rgb(255, 215, 64),
                    )
                };

                painter.rect_filled(badge_rect, 5.0, cor_fundo_badge);
                painter.rect_stroke(
                    badge_rect,
                    5.0,
                    egui::Stroke::new(1.2, cor_borda_badge),
                    egui::StrokeKind::Inside,
                );

                painter.text(
                    meio,
                    egui::Align2::CENTER_CENTER,
                    aresta.peso.to_string(),
                    egui::FontId::proportional(13.0),
                    cor_texto_badge,
                );
            }
        }

        // 5. Desenha os Vértices
        for (i, vertice) in self.grafo.verticu.iter().enumerate().take(self.posicoes.len()) {
            let pos = self.posicoes[i];

            let faz_parte_caminho = self
                .resultado_dijkstra
                .as_ref()
                .map(|r| r.caminho.contains(&i))
                .unwrap_or(false);

            let (cor_preenchimento, cor_borda) = if i == self.no_origem {
                (
                    egui::Color32::from_rgb(39, 174, 96), // Verde Origem
                    egui::Color32::from_rgb(46, 204, 113),
                )
            } else if i == self.no_destino {
                (
                    egui::Color32::from_rgb(192, 57, 43), // Vermelho Destino
                    egui::Color32::from_rgb(231, 76, 60),
                )
            } else if faz_parte_caminho {
                (
                    egui::Color32::from_rgb(26, 120, 75), // Verde Nó do Caminho
                    egui::Color32::from_rgb(46, 204, 113),
                )
            } else {
                (
                    egui::Color32::from_rgb(41, 128, 185), // Azul Padrão
                    egui::Color32::from_rgb(52, 152, 219),
                )
            };

            // Glow externo para nós em destaque
            if i == self.no_origem || i == self.no_destino || faz_parte_caminho {
                painter.circle_stroke(
                    pos,
                    raio_no + 4.5,
                    egui::Stroke::new(2.5, cor_borda.gamma_multiply(0.5)),
                );
            }

            // Círculo principal
            painter.circle_filled(pos, raio_no, cor_preenchimento);
            painter.circle_stroke(pos, raio_no, egui::Stroke::new(2.5, egui::Color32::WHITE));

            // Letra do Vértice
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                vertice.cha_nome.to_string(),
                egui::FontId::proportional(20.0),
                egui::Color32::WHITE,
            );

            // Informações extras abaixo da bolinha (distância calculada e grau)
            let texto_info = if let Some(res) = &self.resultado_dijkstra {
                if let Some(&d) = res.distancias.get(i) {
                    if d == u32::MAX {
                        format!("d: ∞ | g: {}", vertice.grau)
                    } else {
                        format!("d: {} | g: {}", d, vertice.grau)
                    }
                } else {
                    format!("grau: {}", vertice.grau)
                }
            } else {
                format!("grau: {}", vertice.grau)
            };

            painter.text(
                pos + egui::vec2(0.0, raio_no + 11.0),
                egui::Align2::CENTER_CENTER,
                texto_info,
                egui::FontId::proportional(11.0),
                if faz_parte_caminho {
                    egui::Color32::from_rgb(46, 204, 113)
                } else {
                    egui::Color32::from_rgb(160, 170, 190)
                },
            );
        }
    }
}
//************************************************************************************
// Função auxiliar para verificar existência de aresta
fn vertice_tem_aresta(grafo: &Grap, de: usize, para: usize) -> bool {
    if let Some(v) = grafo.verticu.get(de) {
        v.arestu.iter().any(|a| a.dcamin == para)
    } else {
        false
    }
}

//************************************************************************************
fn main(){//o claude mexeu e tirou meu lindos comentários, olha que lindo
            //daq pra baixo a magia acontece
    println!("\ntá compilando caralhooo!!!");

    let grafu = Grap {//A0,B1,C2,D3,E4,F5,G6,H7,I8,J9,
        verticu: vec![
            Vertic { //B, C, F, I
                arestu: vec![
                    Arest { peso: 4, dcamin: 1 }, 
                    Arest { peso: 3, dcamin: 2 }, 
                    Arest { peso: 6, dcamin: 5 }, 
                    Arest { peso: 7, dcamin: 8 }, 
                ],
                grau: 4,
                num_nome: 0,
                cha_nome: 'A',
            },
            Vertic { // A,C,D,I,J
                arestu: vec![
                    Arest { peso: 4, dcamin: 0 }, 
                    Arest { peso: 5, dcamin: 2 }, 
                    Arest { peso: 2, dcamin: 3 }, 
                    Arest { peso: 5, dcamin: 8 }, 
                    Arest { peso: 6, dcamin: 9 }, 
                ],
                grau: 5,
                num_nome: 1,
                cha_nome: 'B',
            },
            Vertic { // A,B,D,E,G,J
                arestu: vec![
                    Arest { peso: 3, dcamin: 0 },
                    Arest { peso: 5, dcamin: 1 },
                    Arest { peso: 1, dcamin: 3 },
                    Arest { peso: 3, dcamin: 4 },
                    Arest { peso: 4, dcamin: 6 },
                    Arest { peso: 8, dcamin: 9 },
                ],
                grau: 6,
                num_nome: 2,
                cha_nome: 'C',
            },
            Vertic { // B,C,E,F,H 
                arestu: vec![
                    Arest { peso: 2, dcamin: 1 },
                    Arest { peso: 1, dcamin: 2 },
                    Arest { peso: 4, dcamin: 4 },
                    Arest { peso: 3, dcamin: 5 },
                    Arest { peso: 5, dcamin: 7 },
                ],
                grau: 5,
                num_nome: 3,
                cha_nome: 'D',
            },
            Vertic { //C,D,G,H
                arestu: vec![
                    Arest { peso: 3, dcamin: 2 }, 
                    Arest { peso: 4, dcamin: 3 }, 
                    Arest { peso: 2, dcamin: 6 }, 
                    Arest { peso: 6, dcamin: 7 }, 
                ],
                grau: 4,
                num_nome: 4,
                cha_nome: 'E',
            },
            Vertic { // 5 = F
                arestu: vec![
                    Arest { peso: 6, dcamin: 0 }, // F -> A
                    Arest { peso: 3, dcamin: 3 }, // F -> D
                    Arest { peso: 4, dcamin: 7 }, // F -> H
                    Arest { peso: 9, dcamin: 8 }, // F -> I
                ],
                grau: 4,
                num_nome: 5,
                cha_nome: 'F',
            },
            Vertic { // 6 = G
                arestu: vec![
                    Arest { peso: 4, dcamin: 2 }, // G -> C
                    Arest { peso: 2, dcamin: 4 }, // G -> E
                    Arest { peso: 5, dcamin: 7 }, // G -> H
                    Arest { peso: 7, dcamin: 9 }, // G -> J
                ],
                grau: 4,
                num_nome: 6,
                cha_nome: 'G',
            },
            Vertic { // 7 = H
                arestu: vec![
                    Arest { peso: 5, dcamin: 3 }, // H -> D
                    Arest { peso: 6, dcamin: 4 }, // H -> E
                    Arest { peso: 4, dcamin: 5 }, // H -> F
                    Arest { peso: 5, dcamin: 6 }, // H -> G
                ],
                grau: 4,
                num_nome: 7,
                cha_nome: 'H',
            },
            Vertic { // 8 = I
                arestu: vec![
                    Arest { peso: 7, dcamin: 0 }, // I -> A
                    Arest { peso: 5, dcamin: 1 }, // I -> B
                    Arest { peso: 9, dcamin: 5 }, // I -> F
                ],
                grau: 3,
                num_nome: 8,
                cha_nome: 'I',
            },
            Vertic { // 9 = J
                arestu: vec![
                    Arest { peso: 6, dcamin: 1 }, 
                    Arest { peso: 8, dcamin: 2 }, 
                    Arest { peso: 7, dcamin: 6 }, 
                ],
                grau: 3,
                num_nome: 9,
                cha_nome: 'J',
            },
        ],
    };

    ver_grap_bonitinho(&grafu);
}
//************************************************************************************
#[allow(dead_code)]
fn printar_grap_em_linha(grafu: &Grap) {
    for vertice in &grafu.verticu {
        println!("\nVertice: {}", vertice.cha_nome);
        for aresta in &vertice.arestu {
            let destino = &grafu.verticu[aresta.dcamin];
            println!("  -------> {} | Peso: {}", destino.cha_nome, aresta.peso);
        }
    }
}
//************************************************************************************
fn ver_grap_bonitinho(grafo: &Grap) -> eframe::Result {
    let opcoes = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([820.0, 620.0])
            .with_min_inner_size([650.0, 480.0])
            .with_title("Visualizador de Grafos - Algoritmo de Dijkstra"),
        ..Default::default()
    };
    eframe::run_native(
        "Visualizador de Grafos",
        opcoes,
        Box::new(|_cc| {
            Ok(Box::new(Aplicacao::new(grafo.clone())))
        }),
    )
}
