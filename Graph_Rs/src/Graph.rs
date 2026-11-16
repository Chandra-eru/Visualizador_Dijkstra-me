use std::io;//bagulho para depois
use eframe::egui;//interface

#[derive(Clone)]
struct Arest{//peso do bagulho e daonde atéaonde
    peso: u32,
    dcamin: usize,
}
#[derive(Clone)]
struct Vertic{
    arestu:Vec<Arest>,
    grau: u32,   
    num_nome: u32,    
    cha_nome: char,   
}
#[derive(Clone)]
struct Grap {// Grafo(Vertice, Arestas)
    verticu:Vec<Vertic>,
}
//************************************************************************************
struct Aplicacao {
    grafo: Grap,
}

impl eframe::App for Aplicacao {
    fn ui(
        &mut self,
        ui: &mut egui::Ui,
        _frame: &mut eframe::Frame,
    ) {
        let id_posicoes = egui::Id::NULL.with("grafo_posicoes");
        let id_arrastando = egui::Id::NULL.with("grafo_arrastando");
        let id_origem = egui::Id::NULL.with("grafo_origem");
        let id_destino = egui::Id::NULL.with("grafo_destino");

        let n = self.grafo.verticu.len();

        // Recupera ou inicializa as posições dos vértices
        let mut posicoes: Vec<egui::Pos2> = ui.ctx().data_mut(|d| {
            d.get_temp(id_posicoes).unwrap_or_else(|| {
                // Layout padrão inicial (5 nós)
                vec![
                    egui::pos2(150.0, 160.0), // A
                    egui::pos2(350.0, 100.0), // B
                    egui::pos2(550.0, 160.0), // C
                    egui::pos2(230.0, 360.0), // D
                    egui::pos2(470.0, 360.0), // E
                ]
            })
        });

        // Garante que o vetor tenha posições suficientes caso o grafo cresça
        while posicoes.len() < n {
            let idx = posicoes.len();
            posicoes.push(egui::pos2(150.0 + (idx as f32) * 80.0, 200.0));
        }

        let mut arrastando: Option<usize> = ui.ctx().data_mut(|d| d.get_temp(id_arrastando));
        let mut no_origem: usize = ui.ctx().data_mut(|d| d.get_temp(id_origem).unwrap_or(0));
        let mut no_destino: usize = ui.ctx().data_mut(|d| d.get_temp(id_destino).unwrap_or(4.min(n.saturating_sub(1))));

        // Barra Superior: Cabeçalho e Seleção de Nós
        ui.horizontal(|ui| {
            ui.heading("🌐 Visualizador de Grafos");
            ui.separator();

            ui.label(egui::RichText::new("Origem:").strong());
            for (i, v) in self.grafo.verticu.iter().enumerate() {
                let selecionado = no_origem == i;
                if ui.selectable_label(selecionado, v.cha_nome.to_string()).clicked() {
                    no_origem = i;
                }
            }

            ui.separator();
            ui.label(egui::RichText::new("Destino:").strong());
            for (i, v) in self.grafo.verticu.iter().enumerate() {
                let selecionado = no_destino == i;
                if ui.selectable_label(selecionado, v.cha_nome.to_string()).clicked() {
                    no_destino = i;
                }
            }

            ui.separator();
            if ui.button("↺ Resetar Layout").clicked() {
                posicoes = vec![
                    egui::pos2(150.0, 160.0),
                    egui::pos2(350.0, 100.0),
                    egui::pos2(550.0, 160.0),
                    egui::pos2(230.0, 360.0),
                    egui::pos2(470.0, 360.0),
                ];
            }
        });

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new("💡 Dica: Arraste os nós para mover. Verde = Origem | Vermelho = Destino")
                    .small()
                    .color(egui::Color32::from_rgb(170, 170, 180)),
            );
        });

        ui.separator();

        // Canvas do Grafo
        let area_disponivel = ui.available_size_before_wrap();
        let (rect, resposta) = ui.allocate_exact_size(area_disponivel, egui::Sense::click_and_drag());
        let painter = ui.painter_at(rect);

        // Fundo escuro com cantos arredondados
        painter.rect(
            rect,
            12.0,
            egui::Color32::from_rgb(18, 20, 26),
            egui::Stroke::new(1.5, egui::Color32::from_rgb(45, 52, 70)),
            egui::StrokeKind::Inside,
        );

        let raio_no = 26.0;

        // Interação do Mouse: Arrastar nós
        if let Some(pos_mouse) = resposta.hover_pos() {
            if resposta.drag_started() {
                for (i, &p) in posicoes.iter().enumerate().take(n) {
                    if p.distance(pos_mouse) <= raio_no {
                        arrastando = Some(i);
                        break;
                    }
                }
            }

            if resposta.dragged() {
                if let Some(idx) = arrastando {
                    if idx < posicoes.len() {
                        let margem = raio_no + 5.0;
                        let novo_x = pos_mouse.x.clamp(rect.min.x + margem, rect.max.x - margem);
                        let novo_y = pos_mouse.y.clamp(rect.min.y + margem, rect.max.y - margem);
                        posicoes[idx] = egui::pos2(novo_x, novo_y);
                    }
                }
            }
        }

        if resposta.drag_stopped() {
            arrastando = None;
        }

        // 1. Desenha as Arestas
        for (origem, vertice) in self.grafo.verticu.iter().enumerate() {
            for aresta in &vertice.arestu {
                let destino = aresta.dcamin;
                if destino >= posicoes.len() || origem >= posicoes.len() {
                    continue;
                }

                // Se for bidirecional, desenha apenas uma vez para não sobrepor o texto do peso
                if origem > destino && vertice_tem_aresta(&self.grafo, destino, origem) {
                    continue;
                }

                let inicio = posicoes[origem];
                let fim = posicoes[destino];

                // Linha da aresta
                painter.line_segment(
                    [inicio, fim],
                    egui::Stroke::new(2.5, egui::Color32::from_rgb(140, 150, 175)),
                );

                // Badge arredondado com o peso no meio da aresta
                let meio = egui::pos2((inicio.x + fim.x) * 0.5, (inicio.y + fim.y) * 0.5);
                let badge_size = egui::vec2(22.0, 18.0);
                let badge_rect = egui::Rect::from_center_size(meio, badge_size);

                painter.rect_filled(badge_rect, 4.0, egui::Color32::from_rgb(30, 34, 45));
                painter.rect_stroke(
                    badge_rect,
                    4.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 80, 105)),
                    egui::StrokeKind::Inside,
                );

                painter.text(
                    meio,
                    egui::Align2::CENTER_CENTER,
                    aresta.peso.to_string(),
                    egui::FontId::proportional(13.0),
                    egui::Color32::from_rgb(255, 215, 64),
                );
            }
        }

        // 2. Desenha os Vértices
        for (i, vertice) in self.grafo.verticu.iter().enumerate().take(posicoes.len()) {
            let pos = posicoes[i];

            // Cores diferenciadas por estado
            let (cor_preenchimento, cor_borda) = if i == no_origem {
                (
                    egui::Color32::from_rgb(39, 174, 96), // Verde Origem
                    egui::Color32::from_rgb(46, 204, 113),
                )
            } else if i == no_destino {
                (
                    egui::Color32::from_rgb(192, 57, 43), // Vermelho Destino
                    egui::Color32::from_rgb(231, 76, 60),
                )
            } else {
                (
                    egui::Color32::from_rgb(41, 128, 185), // Azul Padrão
                    egui::Color32::from_rgb(52, 152, 219),
                )
            };

            // Glow sutil no nó de origem e destino
            if i == no_origem || i == no_destino {
                painter.circle_stroke(
                    pos,
                    raio_no + 4.0,
                    egui::Stroke::new(2.0, cor_borda.gamma_multiply(0.4)),
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

            // Subtexto com Grau do Vértice abaixo da bolinha
            painter.text(
                pos + egui::vec2(0.0, raio_no + 10.0),
                egui::Align2::CENTER_CENTER,
                format!("grau: {}", vertice.grau),
                egui::FontId::proportional(11.0),
                egui::Color32::from_rgb(160, 170, 190),
            );
        }

        // Salva estado interativo
        ui.ctx().data_mut(|d| {
            d.insert_temp(id_posicoes, posicoes);
            d.insert_temp(id_arrastando, arrastando);
            d.insert_temp(id_origem, no_origem);
            d.insert_temp(id_destino, no_destino);
        });
    }
}

// Função auxiliar para evitar desenhar arestas bidirecionais duas vezes
fn vertice_tem_aresta(grafo: &Grap, de: usize, para: usize) -> bool {
    if let Some(v) = grafo.verticu.get(de) {
        v.arestu.iter().any(|a| a.dcamin == para)
    } else {
        false
    }
}
//************************************************************************************
fn main() {
    println!(""); 
    println!("Tá compilando porra");//nao tenho comentários

    let mut grafu = Grap{// A0 B1 C2 D3 E4
        verticu: vec![
            Vertic{
                arestu:vec![
                    Arest{
                        peso:4,
                        dcamin:1,
                    },
                    Arest{
                        peso:3,
                        dcamin:2,
                    },
                ],
                grau:2,
                num_nome:0,
                cha_nome:'A',
            },

            Vertic{
                arestu:vec![
                    Arest{
                        peso:4,
                        dcamin:0,
                    },
                    Arest{
                        peso:5,
                        dcamin:2,
                    },
                    Arest{
                        peso:2,
                        dcamin:3,
                    },

                ],
                grau:3,
                num_nome:1,
                cha_nome:'B',
            },

            Vertic{
                arestu:vec![
                    Arest{
                        peso:3,
                        dcamin:0,
                    },
                    Arest{
                        peso:5,
                        dcamin:1,
                    },
                    Arest{
                        peso:1,
                        dcamin:3,
                    },
                    Arest{
                        peso:3,
                        dcamin:4,
                    },
                ],
                grau:4,
                num_nome:2,
                cha_nome:'C',
            },

            Vertic{
                arestu:vec![
                    Arest{
                        peso:2,
                        dcamin:1,
                    },
                    Arest{
                        peso:1,
                        dcamin:2,
                    },
                    Arest{
                        peso:4,
                        dcamin:4,
                    },
                ],
                grau:3,
                num_nome:3,
                cha_nome:'D',
            },

            Vertic{
                arestu:vec![
                    Arest{
                        peso:3,
                        dcamin:2,
                    },
                    Arest{
                        peso:4,
                        dcamin:3,
                    }
                ],
                grau:2,
                num_nome:4,
                cha_nome:'E',
            },
        ],
    };
//final das structs
    //printar_grap_em_linha(&grafu);
    let aoi = ver_grap_bonitinho(&grafu);    
}
//************************************************************************************
fn printar_grap_em_linha(grafu: &Grap){
        for vertice in &grafu.verticu {
        println!("\nVertice: {}", vertice.cha_nome);
        for aresta in &vertice.arestu {
            let destino = &grafu.verticu[aresta.dcamin];
            println!("  -------> {} | Peso: {}", destino.cha_nome, aresta.peso);
        }
    }
}
//************************************************************************************
fn ver_grap_bonitinho(grafo: &Grap) -> eframe::Result{
        let opcoes = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Visualizador de Grafos",
        opcoes,
        Box::new(|_cc| {
            Ok(Box::new(Aplicacao {
                grafo:grafo.clone(),
            }))
        }),
    )
}
//************************************************************************************
// preciso de uma fila pra dizer qual é o menor valor pro dijkstra, uma fila de prioridade, um array serve 
//fns inserir(colocar coisas no array), remover menor(tirar o menor valor da fila das prioridades), 
//editar(mudar valores do array)

//fn subgrafo_dijk(grafu:&Grap, origem:usize, destino:usize) -> Grap {
//
//}
//
//fn caminho_dijk(grafu:&Grap, origem:usize, destino:usize) -> Vec<usize>{
//
//}