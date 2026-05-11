# Bibliography Research — Geometry of Consolidation in Embedding Memory Systems

All entries verified against primary sources (arXiv, ACL Anthology, IEEE Xplore, JMLR, PubMed, journal pages).
Entries are grouped by topic and ready to paste into a `.bib` file.

---

## Topic 1: Retrieval-Augmented Generation (RAG)

```bibtex
@inproceedings{lewis2020rag,
  title     = {Retrieval-Augmented Generation for Knowledge-Intensive {NLP} Tasks},
  author    = {Lewis, Patrick and Perez, Ethan and Piktus, Aleksandra and Petroni,
               Fabio and Karpukhin, Vladimir and Goyal, Naman and K{\"{u}}ttler,
               Heinrich and Lewis, Mike and Yih, Wen-tau and Rockt{\"{a}}schel, Tim
               and Riedel, Sebastian and Kiela, Douwe},
  booktitle = {Advances in Neural Information Processing Systems},
  volume    = {33},
  pages     = {9459--9474},
  year      = {2020},
  publisher = {Curran Associates, Inc.},
  url       = {https://arxiv.org/abs/2005.11401},
  eprint    = {2005.11401},
  archivePrefix = {arXiv}
}

@inproceedings{guu2020realm,
  title     = {{REALM}: Retrieval-Augmented Language Model Pre-Training},
  author    = {Guu, Kelvin and Lee, Kenton and Tung, Zora and Pasupat, Panupong
               and Chang, Ming-Wei},
  booktitle = {Proceedings of the 37th International Conference on Machine Learning},
  series    = {Proceedings of Machine Learning Research},
  volume    = {119},
  pages     = {3929--3938},
  year      = {2020},
  publisher = {PMLR},
  url       = {https://arxiv.org/abs/2002.08909},
  eprint    = {2002.08909},
  archivePrefix = {arXiv}
}

@inproceedings{karpukhin2020dpr,
  title     = {Dense Passage Retrieval for Open-Domain Question Answering},
  author    = {Karpukhin, Vladimir and O{\u{g}}uz, Barlas and Min, Sewon and
               Lewis, Patrick and Wu, Ledell and Edunov, Sergey and Chen, Danqi
               and Yih, Wen-tau},
  booktitle = {Proceedings of the 2020 Conference on Empirical Methods in
               Natural Language Processing ({EMNLP})},
  pages     = {6769--6781},
  year      = {2020},
  publisher = {Association for Computational Linguistics},
  doi       = {10.18653/v1/2020.emnlp-main.550},
  url       = {https://arxiv.org/abs/2004.04906},
  eprint    = {2004.04906},
  archivePrefix = {arXiv}
}

@article{izacard2023atlas,
  title     = {Atlas: Few-Shot Learning with Retrieval Augmented Language Models},
  author    = {Izacard, Gautier and Lewis, Patrick and Lomeli, Maria and
               Hosseini, Lucas and Petroni, Fabio and Schick, Timo and
               Dwivedi-Yu, Jane and Joulin, Armand and Riedel, Sebastian and
               Grave, Edouard},
  journal   = {Journal of Machine Learning Research},
  volume    = {24},
  number    = {251},
  pages     = {1--43},
  year      = {2023},
  url       = {https://jmlr.org/papers/v24/23-0037.html},
  eprint    = {2208.03299},
  archivePrefix = {arXiv}
}

@inproceedings{borgeaud2022retro,
  title     = {Improving Language Models by Retrieving from Trillions of Tokens},
  author    = {Borgeaud, Sebastian and Mensch, Arthur and Hoffmann, Jordan and
               Cai, Trevor and Rutherford, Eliza and Millican, Katie and van den
               Driessche, George and Lespiau, Jean-Baptiste and Damoc, Bogdan and
               Clark, Aidan and de Las Casas, Diego and Guy, Aurelia and
               Menick, Jacob and Ring, Roman and Hennigan, Tom and Lakhotia,
               Saffron and Huang, Ruibo and Szlam, Arthur and Korhonen, Anna
               and de Freitas, Nathan and others},
  booktitle = {Proceedings of the 39th International Conference on Machine Learning},
  series    = {Proceedings of Machine Learning Research},
  volume    = {162},
  pages     = {2206--2240},
  year      = {2022},
  publisher = {PMLR},
  url       = {https://arxiv.org/abs/2112.04426},
  eprint    = {2112.04426},
  archivePrefix = {arXiv}
}

@inproceedings{izacard2021fid,
  title     = {Leveraging Passage Retrieval with Generative Models for Open
               Domain Question Answering},
  author    = {Izacard, Gautier and Grave, Edouard},
  booktitle = {Proceedings of the 16th Conference of the European Chapter of
               the Association for Computational Linguistics: Main Volume},
  pages     = {874--880},
  year      = {2021},
  publisher = {Association for Computational Linguistics},
  doi       = {10.18653/v1/2021.eacl-main.74},
  url       = {https://aclanthology.org/2021.eacl-main.74},
  eprint    = {2007.01282},
  archivePrefix = {arXiv}
}
```

---

## Topic 2: Vector-Store Quantization and Approximate Nearest Neighbor Search

```bibtex
@article{jegou2011pq,
  title     = {Product Quantization for Nearest Neighbor Search},
  author    = {J{\'e}gou, Herv{\'e} and Douze, Matthijs and Schmid, Cordelia},
  journal   = {{IEEE} Transactions on Pattern Analysis and Machine Intelligence},
  volume    = {33},
  number    = {1},
  pages     = {117--128},
  year      = {2011},
  publisher = {IEEE},
  doi       = {10.1109/TPAMI.2010.57}
}

@inproceedings{ge2013opq,
  title     = {Optimized Product Quantization for Approximate Nearest Neighbor Search},
  author    = {Ge, Tiezheng and He, Kaiming and Ke, Qifa and Sun, Jian},
  booktitle = {{IEEE} Conference on Computer Vision and Pattern Recognition
               ({CVPR})},
  pages     = {2946--2953},
  year      = {2013},
  publisher = {IEEE},
  doi       = {10.1109/CVPR.2013.379}
}

@inproceedings{charikar2002lsh,
  title     = {Similarity Estimation Techniques from Rounding Algorithms},
  author    = {Charikar, Moses S.},
  booktitle = {Proceedings of the 34th Annual {ACM} Symposium on Theory of
               Computing ({STOC})},
  pages     = {380--388},
  year      = {2002},
  publisher = {ACM},
  doi       = {10.1145/509907.509965}
}

@inproceedings{datar2004lsh,
  title     = {Locality-Sensitive Hashing Scheme Based on $p$-Stable
               Distributions},
  author    = {Datar, Mayur and Immorlica, Nicole and Indyk, Piotr and
               Mirrokni, Vahab S.},
  booktitle = {Proceedings of the 20th Annual Symposium on Computational
               Geometry ({SCG})},
  pages     = {253--262},
  year      = {2004},
  publisher = {ACM},
  doi       = {10.1145/1007352.1007450}
}

@article{malkov2018hnsw,
  title     = {Efficient and Robust Approximate Nearest Neighbor Search Using
               Hierarchical Navigable Small World Graphs},
  author    = {Malkov, Yu. A. and Yashunin, D. A.},
  journal   = {{IEEE} Transactions on Pattern Analysis and Machine Intelligence},
  volume    = {42},
  number    = {4},
  pages     = {824--836},
  year      = {2020},
  publisher = {IEEE},
  doi       = {10.1109/TPAMI.2018.2889473},
  url       = {https://arxiv.org/abs/1603.09320},
  eprint    = {1603.09320},
  archivePrefix = {arXiv}
}

@article{johnson2019faiss,
  title     = {Billion-Scale Similarity Search with {GPU}s},
  author    = {Johnson, Jeff and Douze, Matthijs and J{\'e}gou, Herv{\'e}},
  journal   = {{IEEE} Transactions on Big Data},
  volume    = {7},
  number    = {3},
  pages     = {535--547},
  year      = {2021},
  publisher = {IEEE},
  doi       = {10.1109/TBDATA.2019.2921572},
  url       = {https://arxiv.org/abs/1702.08734},
  eprint    = {1702.08734},
  archivePrefix = {arXiv}
}
```

---

## Topic 3: Sentence Encoders / Dense Embedding Models

```bibtex
@inproceedings{reimers2019sbert,
  title     = {Sentence-{BERT}: Sentence Embeddings using {S}iamese {BERT}-Networks},
  author    = {Reimers, Nils and Gurevych, Iryna},
  booktitle = {Proceedings of the 2019 Conference on Empirical Methods in
               Natural Language Processing ({EMNLP})},
  pages     = {3982--3992},
  year      = {2019},
  publisher = {Association for Computational Linguistics},
  doi       = {10.18653/v1/D19-1410},
  url       = {https://arxiv.org/abs/1908.10084},
  eprint    = {1908.10084},
  archivePrefix = {arXiv}
}

@inproceedings{song2020mpnet,
  title     = {{MPN}et: Masked and Permuted Pre-training for Language Understanding},
  author    = {Song, Kaitao and Tan, Xu and Qin, Tao and Lu, Jianfeng and
               Liu, Tie-Yan},
  booktitle = {Advances in Neural Information Processing Systems},
  volume    = {33},
  pages     = {16857--16867},
  year      = {2020},
  publisher = {Curran Associates, Inc.},
  url       = {https://arxiv.org/abs/2004.09297},
  eprint    = {2004.09297},
  archivePrefix = {arXiv}
}

@inproceedings{wang2020minilm,
  title     = {{MiniLM}: Deep Self-Attention Distillation for Task-Agnostic
               Compression of Pre-Trained Transformers},
  author    = {Wang, Wenhui and Wei, Furu and Dong, Li and Bao, Hangbo and
               Yang, Nan and Zhou, Ming},
  booktitle = {Advances in Neural Information Processing Systems},
  volume    = {33},
  pages     = {5776--5788},
  year      = {2020},
  publisher = {Curran Associates, Inc.},
  url       = {https://arxiv.org/abs/2002.10957},
  eprint    = {2002.10957},
  archivePrefix = {arXiv}
}

@article{wang2022e5,
  title     = {Text Embeddings by Weakly-Supervised Contrastive Pre-training},
  author    = {Wang, Liang and Yang, Nan and Huang, Xiaolong and Jiao, Binxing
               and Yang, Linjun and Jiang, Daxin and Majumder, Rangan and
               Wei, Furu},
  journal   = {arXiv preprint arXiv:2212.03533},
  year      = {2022},
  url       = {https://arxiv.org/abs/2212.03533},
  eprint    = {2212.03533},
  archivePrefix = {arXiv}
}

@inproceedings{xiao2023bge,
  title     = {C-Pack: Packed Resources for General {C}hinese Embeddings},
  author    = {Xiao, Shitao and Liu, Zheng and Zhang, Peitian and Muennighoff,
               Niklas and Lian, Defu and Nie, Jian-Yun},
  booktitle = {Proceedings of the 47th International {ACM} {SIGIR} Conference
               on Research and Development in Information Retrieval},
  pages     = {641--649},
  year      = {2024},
  publisher = {ACM},
  doi       = {10.1145/3626772.3657878},
  url       = {https://arxiv.org/abs/2309.07597},
  eprint    = {2309.07597},
  archivePrefix = {arXiv}
}

@article{nussbaum2024nomic,
  title     = {Nomic Embed: Training a Reproducible Long Context Text Embedder},
  author    = {Nussbaum, Zach and Morris, John X. and Duderstadt, Brandon and
               Mulyar, Andriy},
  journal   = {Transactions on Machine Learning Research},
  year      = {2024},
  url       = {https://arxiv.org/abs/2402.01613},
  eprint    = {2402.01613},
  archivePrefix = {arXiv}
}
```

---

## Topic 4: Complementary Learning Systems (CLS) / Memory Consolidation Neuroscience

```bibtex
@article{mcclelland1995cls,
  title     = {Why There Are Complementary Learning Systems in the Hippocampus
               and Neocortex: Insights from the Successes and Failures of
               Connectionist Models of Learning and Memory},
  author    = {McClelland, James L. and McNaughton, Bruce L. and O'Reilly,
               Randall C.},
  journal   = {Psychological Review},
  volume    = {102},
  number    = {3},
  pages     = {419--457},
  year      = {1995},
  publisher = {American Psychological Association},
  doi       = {10.1037/0033-295X.102.3.419}
}

@article{alvarez1994consolidation,
  title     = {Memory Consolidation and the Medial Temporal Lobe: A Simple
               Network Model},
  author    = {Alvarez, Pablo and Squire, Larry R.},
  journal   = {Proceedings of the National Academy of Sciences},
  volume    = {91},
  number    = {15},
  pages     = {7041--7045},
  year      = {1994},
  publisher = {National Academy of Sciences},
  doi       = {10.1073/pnas.91.15.7041}
}

@article{marr1971memory,
  title     = {Simple Memory: A Theory for Archicortex},
  author    = {Marr, D.},
  journal   = {Philosophical Transactions of the Royal Society of London.
               Series B, Biological Sciences},
  volume    = {262},
  number    = {841},
  pages     = {23--81},
  year      = {1971},
  publisher = {The Royal Society},
  doi       = {10.1098/rstb.1971.0078}
}

@article{squire1992hippocampus,
  title     = {Memory and the Hippocampus: A Synthesis from Findings with
               Rats, Monkeys, and Humans},
  author    = {Squire, Larry R.},
  journal   = {Psychological Review},
  volume    = {99},
  number    = {2},
  pages     = {195--231},
  year      = {1992},
  publisher = {American Psychological Association},
  doi       = {10.1037/0033-295X.99.2.195}
}

@article{wilson1994replay,
  title     = {Reactivation of Hippocampal Ensemble Memories During Sleep},
  author    = {Wilson, Matthew A. and McNaughton, Bruce L.},
  journal   = {Science},
  volume    = {265},
  number    = {5172},
  pages     = {676--679},
  year      = {1994},
  publisher = {American Association for the Advancement of Science},
  doi       = {10.1126/science.8036517}
}

@article{oreilly1994hippocampus,
  title     = {Hippocampal Conjunctive Encoding, Storage, and Recall: Avoiding
               a Trade-Off},
  author    = {O'Reilly, Randall C. and McClelland, James L.},
  journal   = {Hippocampus},
  volume    = {4},
  number    = {6},
  pages     = {661--682},
  year      = {1994},
  publisher = {Wiley},
  doi       = {10.1002/hipo.450040605}
}
```

---

## Topic 5: Catastrophic Forgetting / Continual Learning

```bibtex
@incollection{mccloskey1989catastrophic,
  title     = {Catastrophic Interference in Connectionist Networks: The
               Sequential Learning Problem},
  author    = {McCloskey, Michael and Cohen, Neal J.},
  booktitle = {Psychology of Learning and Motivation},
  volume    = {24},
  pages     = {109--165},
  year      = {1989},
  publisher = {Academic Press},
  doi       = {10.1016/S0079-7421(08)60536-8}
}

@article{robins1995rehearsal,
  title     = {Catastrophic Forgetting, Rehearsal and Pseudorehearsal},
  author    = {Robins, Anthony V.},
  journal   = {Connection Science},
  volume    = {7},
  number    = {2},
  pages     = {123--146},
  year      = {1995},
  publisher = {Taylor \& Francis},
  doi       = {10.1080/09540099550039318}
}

@article{kirkpatrick2017ewc,
  title     = {Overcoming Catastrophic Forgetting in Neural Networks},
  author    = {Kirkpatrick, James and Pascanu, Razvan and Rabinowitz, Neil and
               Veness, Joel and Desjardins, Guillaume and Rusu, Andrei A. and
               Milan, Kieran and Quan, John and Ramalho, Tiago and
               Grabska-Barwinska, Agnieszka and Hassabis, Demis and Clopath,
               Claudia and Kumaran, Dharshan and Hadsell, Raia},
  journal   = {Proceedings of the National Academy of Sciences},
  volume    = {114},
  number    = {13},
  pages     = {3521--3526},
  year      = {2017},
  publisher = {National Academy of Sciences},
  doi       = {10.1073/pnas.1611835114},
  url       = {https://arxiv.org/abs/1612.00796},
  eprint    = {1612.00796},
  archivePrefix = {arXiv}
}

@inproceedings{lopezpaz2017gem,
  title     = {Gradient Episodic Memory for Continual Learning},
  author    = {Lopez-Paz, David and Ranzato, Marc'Aurelio},
  booktitle = {Advances in Neural Information Processing Systems},
  volume    = {30},
  pages     = {6467--6476},
  year      = {2017},
  publisher = {Curran Associates, Inc.},
  url       = {https://arxiv.org/abs/1706.08840},
  eprint    = {1706.08840},
  archivePrefix = {arXiv}
}

@inproceedings{chaudhry2019agem,
  title     = {Efficient Lifelong Learning with {A-GEM}},
  author    = {Chaudhry, Arslan and Ranzato, Marc'Aurelio and Rohrbach, Marcus
               and Elhoseiny, Mohamed},
  booktitle = {7th International Conference on Learning Representations ({ICLR})},
  year      = {2019},
  url       = {https://arxiv.org/abs/1812.00420},
  eprint    = {1812.00420},
  archivePrefix = {arXiv}
}

@inproceedings{rebuffi2017icarl,
  title     = {{iCaRL}: Incremental Classifier and Representation Learning},
  author    = {Rebuffi, Sylvestre-Alvise and Kolesnikov, Alexander and Sperl,
               Georg and Lampert, Christoph H.},
  booktitle = {{IEEE} Conference on Computer Vision and Pattern Recognition
               ({CVPR})},
  pages     = {2001--2010},
  year      = {2017},
  publisher = {IEEE},
  doi       = {10.1109/CVPR.2017.587},
  url       = {https://arxiv.org/abs/1611.07725},
  eprint    = {1611.07725},
  archivePrefix = {arXiv}
}
```

---

## Topic 6: Effective Rank / Intrinsic Dimension

```bibtex
@inproceedings{roy2007effectiverank,
  title     = {The Effective Rank: A Measure of Effective Dimensionality},
  author    = {Roy, Olivier and Vetterli, Martin},
  booktitle = {15th European Signal Processing Conference ({EUSIPCO})},
  pages     = {606--610},
  year      = {2007},
  address   = {Poznan, Poland},
  publisher = {EURASIP},
  url       = {https://www.eurasip.org/Proceedings/Eusipco/Eusipco2007/Papers/a5p-h05.pdf}
}
```

---

## Topic 7: Concentration of Measure

```bibtex
@book{milman1986asymptotic,
  title     = {Asymptotic Theory of Finite Dimensional Normed Spaces},
  author    = {Milman, Vitali D. and Schechtman, Gideon},
  series    = {Lecture Notes in Mathematics},
  volume    = {1200},
  year      = {1986},
  publisher = {Springer-Verlag},
  address   = {Berlin},
  doi       = {10.1007/978-3-540-38822-7}
}

@incollection{ball1997convex,
  title     = {An Elementary Introduction to Modern Convex Geometry},
  author    = {Ball, Keith},
  booktitle = {Flavors of Geometry},
  editor    = {Levy, Silvio},
  series    = {{MSRI} Publications},
  volume    = {31},
  pages     = {1--58},
  year      = {1997},
  publisher = {Cambridge University Press},
  address   = {Cambridge},
  url       = {https://library.msri.org/books/Book31/files/ball.pdf}
}

@book{vershynin2018hdp,
  title     = {High-Dimensional Probability: An Introduction with Applications
               in Data Science},
  author    = {Vershynin, Roman},
  year      = {2018},
  publisher = {Cambridge University Press},
  address   = {Cambridge},
  doi       = {10.1017/9781108231596},
  url       = {https://www.math.uci.edu/~rvershyn/papers/HDP-book/HDP-book.html}
}

@book{ledoux2001concentration,
  title     = {The Concentration of Measure Phenomenon},
  author    = {Ledoux, Michel},
  series    = {Mathematical Surveys and Monographs},
  volume    = {89},
  year      = {2001},
  publisher = {American Mathematical Society},
  address   = {Providence, RI},
  doi       = {10.1090/surv/089}
}
```

---

## Topic 8: Berry-Esseen Theorem

```bibtex
@article{berry1941gaussian,
  title     = {The Accuracy of the {G}aussian Approximation to the Sum of
               Independent Variates},
  author    = {Berry, Andrew C.},
  journal   = {Transactions of the American Mathematical Society},
  volume    = {49},
  number    = {1},
  pages     = {122--136},
  year      = {1941},
  publisher = {American Mathematical Society},
  doi       = {10.2307/1990053}
}

@article{esseen1942liapounoff,
  title     = {On the Liapounoff Limit of Error in the Theory of Probability},
  author    = {Esseen, Carl-Gustav},
  journal   = {Arkiv f{\"o}r Matematik, Astronomi och Fysik},
  volume    = {28A},
  number    = {9},
  pages     = {1--19},
  year      = {1942},
  publisher = {Almqvist \& Wiksell}
}
```

---

## Topic 9: Open-Domain QA Benchmarks

```bibtex
@article{kwiatkowski2019nq,
  title     = {Natural Questions: A Benchmark for Question Answering Research},
  author    = {Kwiatkowski, Tom and Palomaki, Jennimaria and Redfield, Olivia and
               Collins, Michael and Parikh, Ankur and Alberti, Chris and
               Epstein, Danielle and Polosukhin, Illia and Devlin, Jacob and
               Lee, Kenton and Toutanova, Kristina and Jones, Llion and
               Kelcey, Matthew and Chang, Ming-Wei and Dai, Andrew M. and
               Uszkoreit, Jakob and Le, Quoc and Petrov, Slav},
  journal   = {Transactions of the Association for Computational Linguistics},
  volume    = {7},
  pages     = {452--466},
  year      = {2019},
  publisher = {MIT Press},
  doi       = {10.1162/tacl_a_00276}
}

@inproceedings{yang2018hotpotqa,
  title     = {{H}otpot{QA}: A Dataset for Diverse, Explainable Multi-hop
               Question Answering},
  author    = {Yang, Zhilin and Qi, Peng and Zhang, Saizheng and Bengio, Yoshua
               and Cohen, William W. and Salakhutdinov, Ruslan and Manning,
               Christopher D.},
  booktitle = {Proceedings of the 2018 Conference on Empirical Methods in
               Natural Language Processing ({EMNLP})},
  pages     = {2369--2380},
  year      = {2018},
  publisher = {Association for Computational Linguistics},
  doi       = {10.18653/v1/D18-1259},
  url       = {https://arxiv.org/abs/1809.09600},
  eprint    = {1809.09600},
  archivePrefix = {arXiv}
}

@inproceedings{mallen2023popqa,
  title     = {When Not to Trust Language Models: Investigating Effectiveness
               of Parametric and Non-Parametric Memories},
  author    = {Mallen, Alex and Asai, Akari and Zhong, Victor and Das, Rajarshi
               and Khashabi, Daniel and Hajishirzi, Hannaneh},
  booktitle = {Proceedings of the 61st Annual Meeting of the Association for
               Computational Linguistics ({ACL})},
  pages     = {9802--9822},
  year      = {2023},
  publisher = {Association for Computational Linguistics},
  doi       = {10.18653/v1/2023.acl-long.546},
  url       = {https://arxiv.org/abs/2212.10511},
  eprint    = {2212.10511},
  archivePrefix = {arXiv}
}

@article{bajaj2016msmarco,
  title     = {{MS} {MARCO}: A Human Generated {MAchine} {R}eading
               {CO}mprehension Dataset},
  author    = {Bajaj, Payal and Campos, Daniel and Craswell, Nick and Deng, Li
               and Gao, Jianfeng and Liu, Xiaodong and Majumder, Rangan and
               McNamara, Andrew and Mitra, Bhaskar and Nguyen, Tri and
               Rosenberg, Mir and Song, Xia and Stoica, Alina and Tiwary,
               Saurabh and Wang, Tong},
  journal   = {arXiv preprint arXiv:1611.09268},
  year      = {2016},
  url       = {https://arxiv.org/abs/1611.09268},
  eprint    = {1611.09268},
  archivePrefix = {arXiv}
}

@inproceedings{rajpurkar2016squad,
  title     = {{SQ}u{AD}: 100,000+ Questions for Machine Comprehension of Text},
  author    = {Rajpurkar, Pranav and Zhang, Jian and Lopyrev, Konstantin and
               Liang, Percy},
  booktitle = {Proceedings of the 2016 Conference on Empirical Methods in
               Natural Language Processing ({EMNLP})},
  pages     = {2383--2392},
  year      = {2016},
  publisher = {Association for Computational Linguistics},
  doi       = {10.18653/v1/D16-1264},
  url       = {https://arxiv.org/abs/1606.05250},
  eprint    = {1606.05250},
  archivePrefix = {arXiv}
}
```

---

## Topic 10: Large Language Models (Llama 3.1) and Inference Systems (vLLM)

```bibtex
@article{grattafiori2024llama3,
  title     = {The {L}lama 3 Herd of Models},
  author    = {Grattafiori, Aaron and Dubey, Abhimanyu and Jauhri, Abhinav and
               Pandey, Abhinav and Kadian, Abhishek and Al-Dahle, Ahmad and
               Lesaint, Aiesha and others},
  journal   = {arXiv preprint arXiv:2407.21783},
  year      = {2024},
  url       = {https://arxiv.org/abs/2407.21783},
  eprint    = {2407.21783},
  archivePrefix = {arXiv}
}

@inproceedings{kwon2023vllm,
  title     = {Efficient Memory Management for Large Language Model Serving
               with {PagedAttention}},
  author    = {Kwon, Woosuk and Li, Zhuohan and Zhuang, Siyuan and Sheng, Ying
               and Zheng, Lianmin and Yu, Cody Hao and Gonzalez, Joseph E. and
               Zhang, Hao and Stoica, Ion},
  booktitle = {Proceedings of the 29th Symposium on Operating Systems
               Principles ({SOSP})},
  pages     = {611--626},
  year      = {2023},
  publisher = {ACM},
  doi       = {10.1145/3600006.3613165},
  url       = {https://arxiv.org/abs/2309.06180},
  eprint    = {2309.06180},
  archivePrefix = {arXiv}
}
```

---

## Topic 11: DRM False-Memory Paradigm

```bibtex
@article{roediger1995drm,
  title     = {Creating False Memories: Remembering Words Not Presented in Lists},
  author    = {Roediger, Henry L. and McDermott, Kathleen B.},
  journal   = {Journal of Experimental Psychology: Learning, Memory, and
               Cognition},
  volume    = {21},
  number    = {4},
  pages     = {803--814},
  year      = {1995},
  publisher = {American Psychological Association},
  doi       = {10.1037/0278-7393.21.4.803}
}
```

---

## Topic 12: FAISS

*(See `johnson2019faiss` under Topic 2 — FAISS is the primary reference for that entry.)*

---

## Topic 13: SimCSE

```bibtex
@inproceedings{gao2021simcse,
  title     = {{SimCSE}: Simple Contrastive Learning of Sentence Embeddings},
  author    = {Gao, Tianyu and Yao, Xingcheng and Chen, Danqi},
  booktitle = {Proceedings of the 2021 Conference on Empirical Methods in
               Natural Language Processing ({EMNLP})},
  pages     = {6894--6910},
  year      = {2021},
  publisher = {Association for Computational Linguistics},
  doi       = {10.18653/v1/2021.emnlp-main.552},
  url       = {https://arxiv.org/abs/2104.08821},
  eprint    = {2104.08821},
  archivePrefix = {arXiv}
}
```

---

## Verification Notes

| Key | Source Verified | Notes |
|-----|----------------|-------|
| `lewis2020rag` | arXiv:2005.11401; NeurIPS 2020 proceedings | 12 authors confirmed |
| `guu2020realm` | PMLR proceedings.mlr.press/v119/guu20a | ICML 2020 confirmed |
| `karpukhin2020dpr` | arXiv:2004.04906; ACL Anthology | EMNLP 2020 confirmed |
| `izacard2023atlas` | jmlr.org/papers/v24/23-0037.html | JMLR 24(251):1–43, 2023 confirmed |
| `borgeaud2022retro` | arXiv:2112.04426; PMLR vol 162 | ICML 2022 confirmed |
| `izacard2021fid` | ACL Anthology 2021.eacl-main.74 | EACL 2021 pp. 874–880 confirmed |
| `jegou2011pq` | IEEE TPAMI 33(1):117–128, 2011 | DOI 10.1109/TPAMI.2010.57 |
| `ge2013opq` | IEEE CVPR 2013 | DOI 10.1109/CVPR.2013.379 |
| `charikar2002lsh` | STOC 2002; OpenAIRE DOI confirmed | DOI 10.1145/509907.509965 |
| `datar2004lsh` | SCG 2004; multiple citing sources | DOI 10.1145/1007352.1007450 |
| `malkov2018hnsw` | IEEE TPAMI 42(4):824–836, 2020 | arXiv:1603.09320 |
| `johnson2019faiss` | arXiv:1702.08734; IEEE Trans. Big Data | Vol 7(3):535–547, 2021 |
| `reimers2019sbert` | arXiv:1908.10084; ACL Anthology | EMNLP 2019 confirmed |
| `song2020mpnet` | arXiv:2004.09297; NeurIPS 2020 | Confirmed |
| `wang2020minilm` | arXiv:2002.10957; NeurIPS 2020 | Confirmed |
| `wang2022e5` | arXiv:2212.03533 | arXiv preprint; no formal venue |
| `xiao2023bge` | arXiv:2309.07597; SIGIR 2024 | Standard BGE citation |
| `nussbaum2024nomic` | arXiv:2402.01613; TMLR 2024 | Confirmed |
| `mcclelland1995cls` | Psych Rev 102(3):419–457 | DOI confirmed |
| `alvarez1994consolidation` | PNAS 91(15):7041–7045 | DOI confirmed |
| `marr1971memory` | Phil Trans R Soc B 262(841):23–81 | DOI confirmed |
| `squire1992hippocampus` | Psych Rev 99(2):195–231 | DOI confirmed |
| `wilson1994replay` | Science 265:676–679 | DOI confirmed |
| `oreilly1994hippocampus` | Hippocampus 4(6):661–682 | DOI confirmed |
| `mccloskey1989catastrophic` | Psych. Learning & Motivation 24:109–165 | Chapter confirmed |
| `robins1995rehearsal` | Connection Science 7(2):123–146 | DOI confirmed |
| `kirkpatrick2017ewc` | PNAS 114(13):3521–3526; arXiv:1612.00796 | DOI confirmed |
| `lopezpaz2017gem` | NeurIPS 2017; arXiv:1706.08840 | Confirmed |
| `chaudhry2019agem` | ICLR 2019; arXiv:1812.00420 | Confirmed |
| `rebuffi2017icarl` | CVPR 2017; arXiv:1611.07725 | DOI confirmed |
| `roy2007effectiverank` | EUSIPCO 2007; EURASIP PDF verified | pp. 606–610 |
| `milman1986asymptotic` | Springer LNM 1200 | Standard reference |
| `ball1997convex` | MSRI Publications Vol. 31 | PDF available at MSRI |
| `vershynin2018hdp` | Cambridge University Press 2018 | DOI confirmed |
| `ledoux2001concentration` | AMS Math Surveys 89 | DOI confirmed |
| `berry1941gaussian` | Trans. AMS 49(1):122–136, 1941 | DOI confirmed |
| `esseen1942liapounoff` | Ark. Mat. Astr. Fys. 28A(9):1–19, 1942 | Confirmed via Wikipedia + multiple citing papers (ISSN 0365-4133) |
| `kwiatkowski2019nq` | TACL 7:452–466, 2019 | DOI confirmed |
| `yang2018hotpotqa` | EMNLP 2018; arXiv:1809.09600 | DOI confirmed |
| `mallen2023popqa` | ACL 2023; arXiv:2212.10511 | DOI confirmed |
| `bajaj2016msmarco` | arXiv:1611.09268 | arXiv preprint; widely cited |
| `rajpurkar2016squad` | EMNLP 2016; arXiv:1606.05250 | DOI confirmed |
| `grattafiori2024llama3` | arXiv:2407.21783 | Meta AI; first author Grattafiori confirmed |
| `kwon2023vllm` | SOSP 2023; arXiv:2309.06180 | DOI confirmed |
| `roediger1995drm` | J. Exp. Psych: LMC 21(4):803–814 | DOI confirmed |
| `gao2021simcse` | EMNLP 2021; arXiv:2104.08821 | DOI confirmed |
