ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=w6A0A3kgnq7mqMH/npwAAAAM;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAAN; loclib_name=1n4148_minimelf;
						li:objects {
						}
						ha:attrib {
							footprint=minimelf
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAADf; loclib_name=2n7002_sot23;
						li:objects {
						}
						ha:attrib {
							footprint=sot23
							li:portmap {
								{G->pcb/pinnum=1}
								{S->pcb/pinnum=2}
								{D->pcb/pinnum=3}
							}
						}
					}
					ha:group.3 {
						uuid=4fVIXHh5Wd2rXZEd1WsAAABw; loclib_name=3mmLED_pth;
						li:objects {
						}
						ha:attrib {
							footprint=LED3
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.4 {
						uuid=4fVIXHh5Wd2rXZEd1WsAAAB/; loclib_name=2n3904_to92;
						li:objects {
						}
						ha:attrib {
							footprint=TO92
							li:portmap {
								{E->pcb/pinnum=1}
								{B->pcb/pinnum=2}
								{C->pcb/pinnum=3}
							}
						}
					}
					ha:group.5 {
						uuid=4fVIXHh5Wd2rXZEd1WsAAACA; loclib_name=2N7000_to92;
						li:objects {
						}
						ha:attrib {
							footprint=TO92.fp
							li:portmap {
								{S->pcb/pinnum=1}
								{G->pcb/pinnum=2}
								{D->pcb/pinnum=3}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=w6A0A3kgnq7mqMH/npwAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.2 {
				uuid=w6A0A3kgnq7mqMH/npwAAAAJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAN;
				x=156000; y=164000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAAK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAO;
						x=16000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAAL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAP;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:text.3 { x1=2000; y1=4000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.4 { x1=2000; y1=7000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=12000; y1=0; x2=10000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=4000; y1=0; x2=6000; y2=0; stroke=sym-decor; }
					ha:line.7 { x1=6000; y1=4000; x2=10000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=10000; y1=0; x2=6000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=6000; y1=4000; x2=6000; y2=-4000; stroke=sym-decor; }
					ha:line.10 { x1=10000; y1=4000; x2=10000; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_pth
					name=LED2
					role=symbol
				}
			}
			ha:group.4 {
				uuid=w6A0A3kgnq7mqMH/npwAAABZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=120000; y=164000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAABa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAABb; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=5000; y1=3000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R2
					role=symbol
					value=220R
				}
			}
			ha:group.5 {
				uuid=w6A0A3kgnq7mqMH/npwAAABi; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=36000; y=168000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAABj; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAABk; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=C200
					name=C1
					role=symbol
					value=100nF
				}
			}
			ha:group.7 {
				uuid=w6A0A3kgnq7mqMH/npwAAAC3; src_uuid=w6A0A3kgnq7mqMH/npwAAAC0;
				x=124000; y=108000;
				li:objects {
					ha:text.1 { x1=-8000; y1=-4000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAC4; src_uuid=w6A0A3kgnq7mqMH/npwAAAC1;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:line.3 { x1=0; y1=0; x2=1200; y2=0; stroke=sym-decor; }
					ha:arc.4 { cx=1600; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:group.5 {
						uuid=w6A0A3kgnq7mqMH/npwAAAC5; src_uuid=w6A0A3kgnq7mqMH/npwAAAC2;
						x=8000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:line.6 { x1=6800; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:arc.7 { cx=6400; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.8 { x1=0; y1=0; x2=0; y2=0; stroke=sym-decor; }
					ha:line.9 { x1=0; y1=0; x2=1200; y2=0; stroke=sym-decor; }
					ha:arc.10 { cx=1600; cy=0; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.11 { x1=1200; y1=3600; x2=6800; y2=3600; stroke=sym-decor; }
					ha:line.12 { x1=1600; y1=8000; x2=6400; y2=8000; stroke=sym-decor; }
					ha:line.13 { x1=1600; y1=8000; x2=1600; y2=7200; stroke=sym-decor; }
					ha:line.14 { x1=6400; y1=8000; x2=6400; y2=7200; stroke=sym-decor; }
					ha:line.15 { x1=4000; y1=3600; x2=4000; y2=8000; stroke=sym-decor; }
				}
				ha:attrib {
					footprint=switchMomentaryPCB.lht
					name=SW1
					role=symbol
				}
			}
			ha:group.8 {
				uuid=w6A0A3kgnq7mqMH/npwAAAC6;
				li:objects {
					ha:line.1 { x1=140000; y1=164000; x2=156000; y2=164000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.11 {
				uuid=w6A0A3kgnq7mqMH/npwAAAC/; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=36000; y=172000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAADA; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.12 {
				uuid=w6A0A3kgnq7mqMH/npwAAADF; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=136000; y=100000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAADG; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.15 {
				uuid=w6A0A3kgnq7mqMH/npwAAADL; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=176000; y=160000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAADM; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.17 {
				uuid=w6A0A3kgnq7mqMH/npwAAADR; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=60000; y=144000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAADS; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.18 {
				uuid=w6A0A3kgnq7mqMH/npwAAADb; src_uuid=iNOQfJpO6hT/HFDFGjoAAACC;
				x=116000; y=140000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAADc; src_uuid=iNOQfJpO6hT/HFDFGjoAAACD;
						x=12000; y=12000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=D
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAADd; src_uuid=iNOQfJpO6hT/HFDFGjoAAACE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=G
							role=terminal
						}
					}
					ha:text.3 { x1=2000; y1=13000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.4 { x1=2000; y1=9000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:arc.5 { cx=11000; cy=3000; r=5500; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.6 { x1=4000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.7 { x1=9000; y1=-1000; x2=9000; y2=1000; stroke=sym-decor; }
					ha:line.8 { x1=9000; y1=2000; x2=9000; y2=4000; stroke=sym-decor; }
					ha:line.9 { x1=9000; y1=5000; x2=9000; y2=7000; stroke=sym-decor; }
					ha:line.10 { x1=9000; y1=3000; x2=12000; y2=3000; stroke=sym-decor; }
					ha:line.11 { x1=9000; y1=0; x2=12000; y2=0; stroke=sym-decor; }
					ha:line.12 { x1=9000; y1=6000; x2=12000; y2=6000; stroke=sym-decor; }
					ha:line.13 { x1=12000; y1=6000; x2=12000; y2=8000; stroke=sym-decor; }
					ha:line.14 { x1=12000; y1=-4000; x2=12000; y2=3000; stroke=sym-decor; }
					ha:line.15 { x1=13000; y1=3000; x2=15000; y2=3000; stroke=sym-decor; }
					ha:line.16 { x1=15000; y1=3000; x2=14000; y2=4000; stroke=sym-decor; }
					ha:line.17 { x1=14000; y1=4000; x2=13000; y2=3000; stroke=sym-decor; }
					ha:line.18 { x1=13000; y1=4000; x2=15000; y2=4000; stroke=sym-decor; }
					ha:line.19 { x1=12000; y1=7000; x2=14000; y2=7000; stroke=sym-decor; }
					ha:line.20 { x1=14000; y1=-1000; x2=12000; y2=-1000; stroke=sym-decor; }
					ha:line.21 { x1=8000; y1=7000; x2=8000; y2=0; stroke=sym-decor; }
					ha:polygon.22 {
						li:outline {
							ha:line { x1=10000; y1=4000; x2=9000; y2=3000; }
							ha:line { x1=9000; y1=3000; x2=10000; y2=2000; }
							ha:line { x1=10000; y1=2000; x2=10000; y2=4000; }
						}
						stroke=sym-decor;
						fill=sym-decor;
					}
					ha:group.23 {
						uuid=w6A0A3kgnq7mqMH/npwAAADe; src_uuid=iNOQfJpO6hT/HFDFGjoAAACF;
						x=12000; y=-4000; rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=S
							role=terminal
						}
					}
					ha:line.24 { x1=14000; y1=-1000; x2=14000; y2=3000; stroke=sym-decor; }
					ha:line.25 { x1=14000; y1=4000; x2=14000; y2=7000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=2N7000_to92
					name=Q1
					role=symbol
				}
			}
			ha:group.19 {
				uuid=w6A0A3kgnq7mqMH/npwAAAEo; src_uuid=w6A0A3kgnq7mqMH/npwAAAEi;
				x=180000; y=76000;
				li:objects {
					ha:text.1 { x1=0; y1=18000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAEp; src_uuid=w6A0A3kgnq7mqMH/npwAAAEj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=w6A0A3kgnq7mqMH/npwAAAEq; src_uuid=w6A0A3kgnq7mqMH/npwAAAEk;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=w6A0A3kgnq7mqMH/npwAAAEr; src_uuid=w6A0A3kgnq7mqMH/npwAAAEl;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=w6A0A3kgnq7mqMH/npwAAAEs; src_uuid=w6A0A3kgnq7mqMH/npwAAAEm;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=w6A0A3kgnq7mqMH/npwAAAEt; src_uuid=w6A0A3kgnq7mqMH/npwAAAEn;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:polygon.7 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=18000; }
							ha:line { x1=0; y1=18000; x2=4000; y2=18000; }
							ha:line { x1=4000; y1=18000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=TYCO_fsm2j.lht
					name=KEYER
					role=symbol
				}
			}
			ha:group.20 {
				uuid=w6A0A3kgnq7mqMH/npwAAAE0; src_uuid=w6A0A3kgnq7mqMH/npwAAAEx;
				x=144000; y=152000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAE1; src_uuid=w6A0A3kgnq7mqMH/npwAAAEy;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=w6A0A3kgnq7mqMH/npwAAAE2; src_uuid=w6A0A3kgnq7mqMH/npwAAAEz;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:polygon.4 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=6000; }
							ha:line { x1=0; y1=6000; x2=4000; y2=6000; }
							ha:line { x1=4000; y1=6000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint={connector(1,2,spacing=200.0mil)}
					name=OD_OUTPUT
					role=symbol
				}
			}
			ha:group.22 {
				uuid=w6A0A3kgnq7mqMH/npwAAAE8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=128000; y=128000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAE9; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.23 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFA; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=60000; y=92000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFB; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.24 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFF; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=160000; y=76000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFG; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=C200
					name=C3
					role=symbol
					value=10nF
				}
			}
			ha:group.25 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFI; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAh;
				x=144000; y=76000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAi;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAj;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=6000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=5000; x2=11000; y2=-5000; stroke=sym-decor; }
					ha:line.7 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=C200
					name=C2
					role=symbol
					value=10nF
				}
			}
			ha:group.26 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=144000; y=52000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.27 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFP;
				li:objects {
					ha:line.1 { x1=144000; y1=52000; x2=144000; y2=56000; stroke=wire; }
					ha:line.6 { x1=176000; y1=56000; x2=176000; y2=76000; stroke=wire; }
					ha:line.7 { x1=144000; y1=56000; x2=176000; y2=56000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.37 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFS;
				x=-8000; y=16000;
				li:objects {
					ha:line.1 { x1=136000; y1=112000; x2=136000; y2=116000; stroke=wire; }
					ha:line.2 { x1=136000; y1=116000; x2=144000; y2=116000; stroke=wire; }
					ha:line.3 { x1=144000; y1=116000; x2=144000; y2=132000; stroke=wire; }
					ha:line.5 { x1=144000; y1=132000; x2=148000; y2=132000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.41 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFT;
				x=-20000; y=20000;
				li:objects {
					ha:line.1 { x1=160000; y1=132000; x2=148000; y2=132000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.44 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFU;
				li:objects {
					ha:line.1 { x1=136000; y1=108000; x2=136000; y2=100000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.47 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFV;
				li:objects {
					ha:line.1 { x1=172000; y1=164000; x2=176000; y2=164000; stroke=wire; }
					ha:line.3 { x1=176000; y1=160000; x2=176000; y2=179000; stroke=wire; }
					ha:line.4 { x1=176000; y1=164000; x2=176000; y2=164000; stroke=junction; }
					ha:line.5 { x1=176000; y1=179000; x2=172000; y2=179000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.50 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFW;
				li:objects {
					ha:line.1 { x1=144000; y1=76000; x2=144000; y2=88000; stroke=wire; }
					ha:line.11 { x1=176000; y1=88000; x2=176000; y2=92000; stroke=wire; }
					ha:line.12 { x1=140000; y1=88000; x2=176000; y2=88000; stroke=wire; }
					ha:line.13 { x1=144000; y1=88000; x2=144000; y2=88000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.73 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFa;
				x=164000; y=120000;
				li:objects {
					ha:arc.1 { cx=2000; cy=-4000; r=6000; sang=270.000000; dang=180.000000; stroke=sym-decor; }
					ha:line.2 { x1=2000; y1=2000; x2=0; y2=2000; stroke=sym-decor; }
					ha:line.3 { x1=0; y1=2000; x2=0; y2=-10000; stroke=sym-decor; }
					ha:line.4 { x1=0; y1=-10000; x2=2000; y2=-10000; stroke=sym-decor; }
					ha:group.5 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFb; src_uuid=w6A0A3kgnq7mqMH/npwAAAFY;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.6 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFc; src_uuid=w6A0A3kgnq7mqMH/npwAAAFZ;
						x=0; y=-8000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-4000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:text.7 { x1=0; y1=2000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					footprint=piezo_5mm_to_7.62mm.rf
					name=PIEZO
					role=symbol
				}
			}
			ha:group.78 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFd;
				li:objects {
					ha:line.1 { x1=36000; y1=172000; x2=36000; y2=168000; stroke=wire; }
					ha:line.2 { x1=36000; y1=168000; x2=88000; y2=168000; stroke=wire; }
					ha:line.3 { x1=88000; y1=168000; x2=88000; y2=172000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.81 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFe;
				li:objects {
					ha:line.1 { x1=36000; y1=148000; x2=36000; y2=144000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.102 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFh; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=160000; y=108000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFi; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:connection.120 {
				li:conn {
					/2/25/1/1
					/2/27/1
					/2/27/7
				}
			}
			ha:connection.125 {
				li:conn {
					/2/44/1
					/2/7/5/1
				}
			}
			ha:group.137 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFj;
				li:objects {
					ha:line.1 { x1=160000; y1=112000; x2=160000; y2=108000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.140 {
				li:conn {
					/2/44/1
					/2/12/1/1
				}
			}
			ha:connection.142 {
				li:conn {
					/2/27/1
					/2/26/1/1
				}
			}
			ha:connection.144 {
				li:conn {
					/2/50/1
					/2/25/2/1
				}
			}
			ha:group.153 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFn; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=120000; y=88000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFo; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFp; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=9000; y1=1000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=1000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R3
					role=symbol
					value=1k
				}
			}
			ha:group.154 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFq; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=120000; y=80000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFr; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAFs; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=9000; y1=1000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=1000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R4
					role=symbol
					value=1k
				}
			}
			ha:group.157 {
				uuid=w6A0A3kgnq7mqMH/npwAAAFt;
				li:objects {
					ha:line.4 { x1=160000; y1=80000; x2=160000; y2=76000; stroke=wire; }
					ha:line.10 { x1=176000; y1=80000; x2=176000; y2=84000; stroke=wire; }
					ha:line.11 { x1=140000; y1=80000; x2=176000; y2=80000; stroke=wire; }
					ha:line.12 { x1=160000; y1=80000; x2=160000; y2=80000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.174 {
				li:conn {
					/2/8/1
					/2/4/1/1
				}
			}
			ha:connection.175 {
				li:conn {
					/2/8/1
					/2/2/2/1
				}
			}
			ha:connection.176 {
				li:conn {
					/2/47/1
					/2/2/1/1
				}
			}
			ha:connection.177 {
				li:conn {
					/2/15/1/1
					/2/47/3
				}
			}
			ha:connection.182 {
				li:conn {
					/2/78/1
					/2/5/2/1
					/2/78/2
				}
			}
			ha:group.186 {
				uuid=w6A0A3kgnq7mqMH/npwAAAGk; src_uuid=w6A0A3kgnq7mqMH/npwAAAGd;
				x=84000; y=176000; rot=90.000000; mirx=1;
				li:objects {
					ha:text.1 { x1=4000; y1=22000; rot=270.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAGl; src_uuid=w6A0A3kgnq7mqMH/npwAAAGe;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=w6A0A3kgnq7mqMH/npwAAAGm; src_uuid=w6A0A3kgnq7mqMH/npwAAAGf;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=w6A0A3kgnq7mqMH/npwAAAGn; src_uuid=w6A0A3kgnq7mqMH/npwAAAGg;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=w6A0A3kgnq7mqMH/npwAAAGo; src_uuid=w6A0A3kgnq7mqMH/npwAAAGh;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							footprint={connector(2,3,sequence=pivot,silkmark=external)}
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=w6A0A3kgnq7mqMH/npwAAAGp; src_uuid=w6A0A3kgnq7mqMH/npwAAAGi;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=w6A0A3kgnq7mqMH/npwAAAGq; src_uuid=w6A0A3kgnq7mqMH/npwAAAGj;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint={connector(2,3,sequence=pivot,silkmark=external)}
					name=ISP
					role=symbol
				}
			}
			ha:connection.188 {
				li:conn {
					/2/78/1
					/2/11/1/1
				}
			}
			ha:connection.189 {
				li:conn {
					/2/81/1
					/2/5/1/1
				}
			}
			ha:connection.211 {
				li:conn {
					/2/19/4/1
					/2/157/10
				}
			}
			ha:connection.215 {
				li:conn {
					/2/157/4
					/2/24/2/1
				}
			}
			ha:connection.216 {
				li:conn {
					/2/154/1/1
					/2/157/11
				}
			}
			ha:connection.217 {
				li:conn {
					/2/153/1/1
					/2/50/12
				}
			}
			ha:connection.218 {
				li:conn {
					/2/24/1/1
					/2/27/7
				}
			}
			ha:group.243 {
				uuid=w6A0A3kgnq7mqMH/npwAAAIo;
				x=60000; y=136000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIp; src_uuid=w6A0A3kgnq7mqMH/npwAAAGe;
						x=12000; y=-8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.2 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIq; src_uuid=w6A0A3kgnq7mqMH/npwAAAGf;
						x=12000; y=-12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.3 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIr; src_uuid=w6A0A3kgnq7mqMH/npwAAAGg;
						x=12000; y=-16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.4 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIs; src_uuid=w6A0A3kgnq7mqMH/npwAAAGh;
						x=0; y=-40000; rot=90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.5 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIt; src_uuid=w6A0A3kgnq7mqMH/npwAAAGi;
						x=12000; y=-20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.6 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIu; src_uuid=w6A0A3kgnq7mqMH/npwAAAGj;
						x=12000; y=-24000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:group.7 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIv; src_uuid=w6A0A3kgnq7mqMH/npwAAAGj;
						x=12000; y=-28000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=7
							role=terminal
						}
					}
					ha:group.8 {
						uuid=w6A0A3kgnq7mqMH/npwAAAIw; src_uuid=w6A0A3kgnq7mqMH/npwAAAGj;
						rot=90.000000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=8
							role=terminal
						}
					}
					ha:line.9 { x1=-8000; y1=0; x2=8000; y2=0; stroke=sym-decor; }
					ha:line.10 { x1=8000; y1=0; x2=8000; y2=-36000; stroke=sym-decor; }
					ha:line.11 { x1=8000; y1=-36000; x2=-8000; y2=-36000; stroke=sym-decor; }
					ha:line.12 { x1=-8000; y1=-36000; x2=-8000; y2=0; stroke=sym-decor; }
					ha:text.13 { x1=-4000; y1=-9000; dyntext=0; stroke=sym-decor; text=RESET/PB5; }
					ha:text.14 { x1=3000; y1=-13000; dyntext=0; stroke=sym-decor; text=PB3; }
					ha:text.15 { x1=3000; y1=-17000; dyntext=0; stroke=sym-decor; text=PB4; }
					ha:text.16 { x1=-3000; y1=-21000; dyntext=0; stroke=sym-decor; text=MOSI/PB0; }
					ha:text.17 { x1=-3000; y1=-25000; dyntext=0; stroke=sym-decor; text=MISO/PB1; }
					ha:text.18 { x1=-3000; y1=-29000; dyntext=0; stroke=sym-decor; text=SCK/PB2; }
					ha:text.19 { x1=-2000; y1=-35000; dyntext=0; stroke=sym-decor; text=GND; }
					ha:text.20 { x1=-1000; y1=-4000; dyntext=0; stroke=sym-decor; text=Vcc; }
					ha:text.21 { x1=-8000; y1=0; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					footprint=dip(8)
					name=U1
					role=symbol
				}
			}
			ha:group.244 {
				uuid=w6A0A3kgnq7mqMH/npwAAAIx;
				li:objects {
					ha:line.1 { x1=60000; y1=144000; x2=60000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.245 {
				li:conn {
					/2/244/1
					/2/17/1/1
				}
			}
			ha:connection.246 {
				li:conn {
					/2/244/1
					/2/243/8/1
				}
			}
			ha:group.247 {
				uuid=w6A0A3kgnq7mqMH/npwAAAIy;
				li:objects {
					ha:line.1 { x1=60000; y1=96000; x2=60000; y2=92000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.248 {
				li:conn {
					/2/247/1
					/2/23/1/1
				}
			}
			ha:connection.249 {
				li:conn {
					/2/247/1
					/2/243/4/1
				}
			}
			ha:connection.251 {
				li:conn {
					/2/276/2
					/2/4/2/1
				}
			}
			ha:group.253 {
				uuid=w6A0A3kgnq7mqMH/npwAAAI0;
				li:objects {
					ha:line.1 { x1=72000; y1=128000; x2=112000; y2=128000; stroke=wire; }
					ha:line.2 { x1=112000; y1=128000; x2=112000; y2=140000; stroke=wire; }
					ha:line.4 { x1=100000; y1=172000; x2=100000; y2=128000; stroke=wire; }
					ha:line.5 { x1=100000; y1=128000; x2=100000; y2=128000; stroke=junction; }
					ha:line.6 { x1=112000; y1=140000; x2=116000; y2=140000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.254 {
				li:conn {
					/2/253/1
					/2/243/1/1
				}
			}
			ha:group.256 {
				uuid=w6A0A3kgnq7mqMH/npwAAAI1;
				li:objects {
					ha:line.6 { x1=72000; y1=120000; x2=136000; y2=120000; stroke=wire; }
					ha:line.7 { x1=136000; y1=120000; x2=160000; y2=120000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:group.259 {
				uuid=w6A0A3kgnq7mqMH/npwAAAI2;
				li:objects {
					ha:line.1 { x1=120000; y1=108000; x2=112000; y2=108000; stroke=wire; }
					ha:line.2 { x1=112000; y1=108000; x2=112000; y2=116000; stroke=wire; }
					ha:line.6 { x1=112000; y1=116000; x2=72000; y2=116000; stroke=wire; }
					ha:line.9 { x1=96000; y1=172000; x2=96000; y2=116000; stroke=wire; }
					ha:line.10 { x1=96000; y1=116000; x2=96000; y2=116000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.260 {
				li:conn {
					/2/259/1
					/2/7/2/1
				}
			}
			ha:group.262 {
				uuid=w6A0A3kgnq7mqMH/npwAAAI3;
				li:objects {
					ha:line.6 { x1=96000; y1=112000; x2=72000; y2=112000; stroke=wire; }
					ha:line.10 { x1=96000; y1=112000; x2=96000; y2=88000; stroke=wire; }
					ha:line.12 { x1=96000; y1=88000; x2=120000; y2=88000; stroke=wire; }
					ha:line.13 { x1=84000; y1=172000; x2=84000; y2=112000; stroke=wire; }
					ha:line.14 { x1=84000; y1=112000; x2=84000; y2=112000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.269 {
				li:conn {
					/2/243/3/1
					/2/256/6
				}
			}
			ha:connection.273 {
				li:conn {
					/2/243/6/1
					/2/262/6
				}
			}
			ha:connection.275 {
				li:conn {
					/2/243/5/1
					/2/259/6
				}
			}
			ha:group.276 {
				uuid=w6A0A3kgnq7mqMH/npwAAAI9;
				li:objects {
					ha:line.1 { x1=80000; y1=124000; x2=72000; y2=124000; stroke=wire; }
					ha:line.2 { x1=120000; y1=164000; x2=80000; y2=164000; stroke=wire; }
					ha:line.3 { x1=80000; y1=164000; x2=80000; y2=124000; stroke=wire; }
					ha:line.7 { x1=114000; y1=164000; x2=114000; y2=179000; stroke=wire; }
					ha:line.8 { x1=114000; y1=164000; x2=114000; y2=164000; stroke=junction; }
					ha:line.9 { x1=114000; y1=179000; x2=120000; y2=179000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.277 {
				li:conn {
					/2/276/1
					/2/243/2/1
				}
			}
			ha:connection.279 {
				li:conn {
					/2/78/3
					/2/186/3/1
				}
			}
			ha:connection.281 {
				li:conn {
					/2/259/9
					/2/186/5/1
				}
			}
			ha:group.283 {
				uuid=w6A0A3kgnq7mqMH/npwAAAI+;
				li:objects {
					ha:line.1 { x1=104000; y1=172000; x2=104000; y2=156000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.284 {
				li:conn {
					/2/283/1
					/2/186/7/1
				}
			}
			ha:group.285 {
				uuid=w6A0A3kgnq7mqMH/npwAAAJB; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=104000; y=156000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAJC; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:connection.286 {
				li:conn {
					/2/285/1/1
					/2/283/1
				}
			}
			ha:group.288 {
				uuid=w6A0A3kgnq7mqMH/npwAAAJX; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=36000; y=54000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAJY; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.289 {
				uuid=w6A0A3kgnq7mqMH/npwAAAJZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=36000; y=144000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAJa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:group.290 {
				uuid=w6A0A3kgnq7mqMH/npwAAAJb; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB6;
				x=36000; y=78000;
				li:objects {
					ha:group.1 {
						uuid=w6A0A3kgnq7mqMH/npwAAAJc; src_uuid=iNOQfJpO6hT/HFDFGjoAAAB7;
						rot=270.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=2500; y1=4000; x2=-2500; y2=4000; stroke=sym-decor; }
					ha:text.3 { x1=-4000; y1=4000; x2=4000; y2=7000; halign=center; dyntext=0; stroke=sym-primary; text=Vcc; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:Vcc}
					}
					role=symbol
				}
			}
			ha:group.291 {
				uuid=w6A0A3kgnq7mqMH/npwAAAJd; src_uuid=w6A0A3kgnq7mqMH/npwAAAJP;
				x=44000; y=64000;
				li:objects {
					ha:line.1 { x1=-4000; y1=10000; x2=-8000; y2=10000; stroke=wire; }
					ha:line.2 { x1=-8000; y1=10000; x2=-8000; y2=14000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.293 {
				li:conn {
					/2/291/2
					/2/290/1/1
				}
			}
			ha:connection.297 {
				li:conn {
					/2/289/1/1
					/2/81/1
				}
			}
			ha:group.306 {
				uuid=w6A0A3kgnq7mqMH/npwAAAJg;
				li:objects {
					ha:line.2 { x1=92000; y1=108000; x2=92000; y2=108000; stroke=junction; }
					ha:line.3 { x1=92000; y1=172000; x2=92000; y2=108000; stroke=wire; }
					ha:line.4 { x1=92000; y1=80000; x2=120000; y2=80000; stroke=wire; }
					ha:line.5 { x1=92000; y1=108000; x2=92000; y2=80000; stroke=wire; }
					ha:line.6 { x1=72000; y1=108000; x2=92000; y2=108000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.307 {
				li:conn {
					/2/306/3
					/2/186/4/1
				}
			}
			ha:connection.308 {
				li:conn {
					/2/306/4
					/2/154/2/1
				}
			}
			ha:connection.309 {
				li:conn {
					/2/306/6
					/2/243/7/1
				}
			}
			ha:connection.317 {
				li:conn {
					/2/153/2/1
					/2/262/12
				}
			}
			ha:connection.318 {
				li:conn {
					/2/19/2/1
					/2/27/6
				}
			}
			ha:connection.319 {
				li:conn {
					/2/19/3/1
					/2/157/10
					/2/157/11
				}
			}
			ha:connection.320 {
				li:conn {
					/2/19/5/1
					/2/50/11
					/2/50/12
				}
			}
			ha:connection.321 {
				li:conn {
					/2/19/6/1
					/2/50/11
				}
			}
			ha:group.322 {
				uuid=4fVIXHh5Wd2rXZEd1WsAAAB4; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAN;
				x=156000; y=179000;
				li:objects {
					ha:group.1 {
						uuid=4fVIXHh5Wd2rXZEd1WsAAAB5; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAO;
						x=16000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4fVIXHh5Wd2rXZEd1WsAAAB6; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAP;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
						}
					}
					ha:text.3 { x1=2000; y1=4000; dyntext=1; stroke=sym-secondary; text=%../a.devmap%; floater=1; }
					ha:text.4 { x1=2000; y1=7000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.5 { x1=12000; y1=0; x2=10000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=4000; y1=0; x2=6000; y2=0; stroke=sym-decor; }
					ha:line.7 { x1=6000; y1=4000; x2=10000; y2=0; stroke=sym-decor; }
					ha:line.8 { x1=10000; y1=0; x2=6000; y2=-4000; stroke=sym-decor; }
					ha:line.9 { x1=6000; y1=4000; x2=6000; y2=-4000; stroke=sym-decor; }
					ha:line.10 { x1=10000; y1=4000; x2=10000; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_pth
					name=LED1
					role=symbol
				}
			}
			ha:group.323 {
				uuid=4fVIXHh5Wd2rXZEd1WsAAAB7; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=120000; y=179000;
				li:objects {
					ha:group.1 {
						uuid=4fVIXHh5Wd2rXZEd1WsAAAB8; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=4fVIXHh5Wd2rXZEd1WsAAAB9; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=5000; y1=3000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=5000; y1=6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=acy(300)
					name=R1
					role=symbol
					value=220R
				}
			}
			ha:group.324 {
				uuid=4fVIXHh5Wd2rXZEd1WsAAAB+; src_uuid=w6A0A3kgnq7mqMH/npwAAAC6;
				x=0; y=15000;
				li:objects {
					ha:line.1 { x1=140000; y1=164000; x2=156000; y2=164000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.325 {
				li:conn {
					/2/324/1
					/2/322/2/1
				}
			}
			ha:connection.326 {
				li:conn {
					/2/324/1
					/2/323/1/1
				}
			}
			ha:connection.327 {
				li:conn {
					/2/276/9
					/2/323/2/1
				}
			}
			ha:connection.328 {
				li:conn {
					/2/47/5
					/2/322/1/1
				}
			}
			ha:connection.330 {
				li:conn {
					/2/262/13
					/2/186/2/1
				}
			}
			ha:connection.331 {
				li:conn {
					/2/253/4
					/2/186/6/1
				}
			}
			ha:group.332 {
				uuid=KfhMIzGC+VuYB5edryMAAADN; src_uuid=KfhMIzGC+VuYB5edryMAAADG;
				x=44000; y=74000; miry=1;
				li:objects {
					ha:text.1 { x1=0; y1=-6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:group.2 {
						uuid=KfhMIzGC+VuYB5edryMAAADO; src_uuid=KfhMIzGC+VuYB5edryMAAADH;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:group.3 {
						uuid=KfhMIzGC+VuYB5edryMAAADP; src_uuid=KfhMIzGC+VuYB5edryMAAADI;
						x=0; y=4000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.4 {
						uuid=KfhMIzGC+VuYB5edryMAAADQ; src_uuid=KfhMIzGC+VuYB5edryMAAADJ;
						x=0; y=8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:group.5 {
						uuid=KfhMIzGC+VuYB5edryMAAADR; src_uuid=KfhMIzGC+VuYB5edryMAAADK;
						x=0; y=12000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:group.6 {
						uuid=KfhMIzGC+VuYB5edryMAAADS; src_uuid=KfhMIzGC+VuYB5edryMAAADL;
						x=0; y=16000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=5
							role=terminal
						}
					}
					ha:group.7 {
						uuid=KfhMIzGC+VuYB5edryMAAADT; src_uuid=KfhMIzGC+VuYB5edryMAAADM;
						x=0; y=20000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=6
							role=terminal
						}
					}
					ha:polygon.8 {
						li:outline {
							ha:line { x1=0; y1=-2000; x2=0; y2=22000; }
							ha:line { x1=0; y1=22000; x2=4000; y2=22000; }
							ha:line { x1=4000; y1=22000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=0; y2=-2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					footprint=TypeBpthUSBvertical.lht
					name=USB
					role=symbol
				}
			}
			ha:connection.333 {
				li:conn {
					/2/332/2/1
					/2/291/1
				}
			}
			ha:group.338 {
				uuid=KfhMIzGC+VuYB5edryMAAADV;
				li:objects {
					ha:line.2 { x1=36000; y1=54000; x2=40000; y2=54000; stroke=wire; }
					ha:line.3 { x1=40000; y1=54000; x2=40000; y2=58000; stroke=wire; }
					ha:line.4 { x1=40000; y1=58000; x2=40000; y2=62000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.340 {
				li:conn {
					/2/332/5/1
					/2/338/4
				}
			}
			ha:connection.341 {
				li:conn {
					/2/338/3
					/2/332/6/1
					/2/338/4
				}
			}
			ha:connection.342 {
				li:conn {
					/2/338/3
					/2/332/7/1
					/2/338/2
				}
			}
			ha:connection.344 {
				li:conn {
					/2/288/1/1
					/2/338/2
				}
			}
			ha:connection.350 {
				li:conn {
					/2/137/1
					/2/102/1/1
				}
			}
			ha:connection.351 {
				li:conn {
					/2/137/1
					/2/73/6/1
				}
			}
			ha:connection.354 {
				li:conn {
					/2/73/5/1
					/2/256/7
				}
			}
			ha:connection.356 {
				li:conn {
					/2/37/1
					/2/18/23/1
					/2/37/2
				}
			}
			ha:connection.357 {
				li:conn {
					/2/37/1
					/2/22/1/1
				}
			}
			ha:connection.359 {
				li:conn {
					/2/41/1
					/2/20/2/1
				}
			}
			ha:connection.360 {
				li:conn {
					/2/41/1
					/2/18/1/1
				}
			}
			ha:connection.361 {
				li:conn {
					/2/253/6
					/2/18/2/1
				}
			}
			ha:connection.362 {
				li:conn {
					/2/37/5
					/2/20/3/1
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 2
     grid = 4.0960 mm
    }
   }
  }
}
