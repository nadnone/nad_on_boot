#![no_std] // pas de bibliothèque standard
#![no_main] // pas de point d'entré 

use core::panic::PanicInfo;

// En cas de kernel panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    loop {}
}

// texte ASCII qui sera affiché
static TEXTE: &[u8] = b"Bienvenue sur le NadKernel o_=";

// C pour Calling function
#[no_mangle]
pub extern  "C" fn _start() -> ! {
    
    // 0xB8000 est l'addresse du tableau graphique vidéo
    // on cast l'adresse héxadécimale en 8bit non signé mutable
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TEXTE.iter().enumerate() {

        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // on écrit byte après byte les lettres ASCII
            *vga_buffer.offset(i as isize * 2 + 1) = 0x2; // on affiche le texte en vert lumineux (comme dans Matrix :,D )

        }

    }

    loop {}

}