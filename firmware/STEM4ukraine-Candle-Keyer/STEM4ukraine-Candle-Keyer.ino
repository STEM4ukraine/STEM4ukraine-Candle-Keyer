/*
 *
 *   Copyright (C) 2024 STEM4ukraine
 *   Website https://github.com/STEM4ukraine
 *   
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 3 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License at <http://www.gnu.org/licenses/> 
 *   for more details.
 *
 */

#include <avr/pgmspace.h>

// this uses https://github.com/MCUdude/MicroCore
// which can be accessed in the boards manager by adding
// https://mcudude.github.io/MicroCore/package_MCUdude_MicroCore_index.json

// the code has been uploaded using an old USB-ASP programmer clone
// and the "USB-ASP slow (Microcore)" setting in the arduino IDE

// Microcore attiny13 selected
// using 1.2MHz internal oscillator
// EEPROM retained
// no bootloader

#define LFSR_INIT1 0xfeed  // non-zero seed value for Galois LFSR used to generate the white noise
#define LFSR_MASK 0x8016   // polynomial from http://users.ece.cmu.edu/~koopman/lfsr/

// note period index is encoded as 4 MSB
// note period index is used to index into a progmem array storing the needed values
// note duration is encoded as 4 LSB
// dit uses the shortest possible note duration
// dah uses the shortest possible note duration times 3

#define DIT_DELAY 112 // 7*16 + 0
#define A_2 107  //6*16 + 11 // 568  // freq 880
#define A_1 103  //6*16 + 7  // 568  // freq 880
#define A_0 99   //6*16 + 3  // 568  // freq 880
#define G1  87   //5*16 + 7  // 638  // freq 784
#define G0  83   //5*16 + 3  // 638  // freq 784
#define FS3 79   //4*16 + 15 // 676  // freq 740
#define FS2 75   //4*16 + 11 // 676  // freq 740
#define FS1 71   //4*16 + 7  // 676  // freq 740
#define FS0 67   //4*16 + 3  // 676  // freq 740
#define FSDAH 66 //4*16 + 2  // 676  // freq 740 DAH F Sharp
#define FSDIT 64 //4*16 + 0  // 676  // freq 740 DIT F Sharp
#define E1  55   //3*16 + 7  // 758  // freq 659
#define E0  51   //3*16 + 3  // 758  // freq 659
#define D1  39   //2*16 + 7  // 851  // freq 587
#define D0  35   //2*16 + 3  // 851  // freq 587
#define CS1 23   //1*16 + 7  // 902  // freq 554
#define CS0 19   //1*16 + 3  // 902  // freq 554
#define B3  15   //0*16 + 15 // 1012 // freq 494
#define B2  11   //0*16 + 11 // 1012 // freq 494
#define B0  3    //0*16 + 3  // 1012 // freq 494

#define NOTE_DELAY_MS 10 // delay between notes

static const uint16_t note_periods[8] PROGMEM = {1012, 902, 851, 758, 676, 638, 568, 500}; // use 500 to denote pause

// we can re-use the music routine to play the dits and dahs as well
static const uint8_t dit_dah[3] PROGMEM = {FSDIT, FSDAH, DIT_DELAY}; // use F sharp for morse sounder of duration 1:3

// first 8 bars
static const uint8_t notes12PD[18] PROGMEM = {FS2, FS0, FS0, E0, FS0, G0, A_2, G0, FS1, E1, D1, FS1, CS1, FS1, B2, CS0, D1, E1};

// 9th-12th bars + 13 into next 3 bars
static const uint8_t notes34PD[18] PROGMEM = {CS1, CS1, FS0, E0, D0, CS0, B0, CS0, D0, B0, CS1, CS1, D1, D1, E1, E1, FS3, FS3};

// final 9 notes of 15-16th bars
static const uint8_t notes4PD[9] PROGMEM = {FS1, CS1, FS1, B2, CS0, D0, E0, FS0, G0};

// next 4 bars + repeat first 11 into next 3 bars
static const uint8_t notes56PD[18] PROGMEM = {A_2, A_0, A_1, FS1, E1, E1, A_0, G0, FS0, E0, D1, D1, E1, E1, FS2, E0, FS1, G1};

// final 5 notes = end of 7th, 8th bar, i.e. index 
static const uint8_t notes6PD[5] PROGMEM = {FS1, CS1, FS1, B3, B3};


uint8_t button = 0, last_key = 0;
uint8_t dit = 0, dah = 0;

void setup() {
  pinMode(0, INPUT_PULLUP); // BUTTON
  pinMode(1, INPUT_PULLUP); // DIT
  pinMode(2, INPUT_PULLUP); // DAH
  pinMode(3, OUTPUT);       // LED1
  pinMode(4, OUTPUT);       // PIEZO
  pinMode(5, INPUT);        // this can be used as an OUTPUT for FET output but only if RESET pin is disabled
}

byte poll_buttons() {
  button = button|!(digitalRead(0));
  dit = dit|!(digitalRead(1));
  dah = dah|!(digitalRead(2));
  return (button | dit | dah);
}

void play_note(const uint8_t * rawnote) {
  int periods, button;
  uint8_t k, raw_note, note_dur;
  uint16_t note_per, cycles, cycles_;

  raw_note = pgm_read_byte(rawnote);
  note_dur = raw_note & 0b00001111;
  periods = ((raw_note>>4) & 0b00001111);
  memcpy_P (&note_per, note_periods+periods, 2);

  // this would need modification for notes outside of the B4-A5 note range
  cycles = (16250)/note_per; // NB avoid overflow for B4-A5 note range. A numerator closer to 65000 would provide more accuracy, but complicate dit/dah timing
  for (k = 0; k < (note_dur + 1); k++) {
    for (int i = 0; i < 3; i++) { // this, times overflow constrained cycles
      cycles_ = cycles;           // gives required duration
      while (cycles_ > 0) {
        if (note_per != 500) { // 500 used to denote delay
          PORTB |= (1 << PB4);
        }
        delayMicroseconds(note_per);
        if (note_per != 500) { // 500 used to denote delay
          PORTB &= ~(1 << PB4);
        }
        delayMicroseconds(note_per);
        cycles_--;
      }
    }
  }
  poll_buttons();
}

void play_dit() {
  play_note(&dit_dah[0]);
  play_note(&dit_dah[2]); // delay
}

void play_dah() {
  play_note(&dit_dah[1]);
  play_note(&dit_dah[2]); // delay
}

void play_anthem() { // breaks if button pressed
  char j, raw_note;
  uint16_t note_dur, note_per;

  // play first 18 notes and then the first 14 again
  for (j = 0; j < 32 && !poll_buttons(); j++) {
    play_note(notes12PD+(j%18));
    delay(NOTE_DELAY_MS);
  }

  // play last note twice
  for (j = 3; j < 5 && !poll_buttons(); j++) {
    play_note(notes6PD+j);
    delay(NOTE_DELAY_MS);
  }

  for (j = 0; j < 31 && !poll_buttons(); j++) {
    play_note(notes34PD+(j%18));
    delay(NOTE_DELAY_MS);
  }

  for (j = 0; j < 9 && !poll_buttons(); j++) {
    play_note(notes4PD+j);
    delay(NOTE_DELAY_MS);
  }

  for (j = 0; j < 29 && !poll_buttons(); j++) {
    play_note(notes56PD+(j%18));
    delay(NOTE_DELAY_MS);
  }

  for (j = 0; j < 5 && !poll_buttons(); j++) {
    play_note(notes6PD+j);
    delay(NOTE_DELAY_MS);
  }

}


// main loop polls the button, and the dit and dah paddles
// lights flicker until a button or paddle has been pressed
void loop() {

  static uint16_t lfsr1 = LFSR_INIT1; 

  while(1) {
    while(!(dit | dah) && !poll_buttons()) { // flicker LEDs while nothing triggered
      if(lfsr1 & 1) {
         lfsr1 =  (lfsr1 >>1) ^ LFSR_MASK;
         PORTB |= (1 << PB3);
      } else {
         lfsr1 >>= 1;
         PORTB &= ~(1 << PB3);
      }
      if(poll_buttons()) break;
      delayMicroseconds(4000);
      if(poll_buttons()) break;
      delayMicroseconds(4000);
      if(poll_buttons()) break;
      delayMicroseconds(4000);
      if(poll_buttons()) break;
      delayMicroseconds(4000);
    }

    if(dit && dah) { // iambic keying
      if(last_key == 1) {
        last_key = 2;
        play_dah();
        dah = 0;
      } else {
        last_key = 1;
        play_dit();
        dit = 0;
      }
    } else if(dit) { // dit keyed
      play_dit();
      last_key = 1;
      dit = 0;
    } else if (dah) { // dah keyed
      play_dah();
      last_key = 2;
      dah = 0;
    } else if (button) { // button pressed
      button = 0;
      last_key = 0;
      delayMicroseconds(80000);
      play_anthem();
      button = 0; // in case button pressed to stop anthem
      delayMicroseconds(80000);
    }

  }
}
