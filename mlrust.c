#include <stdio.h>
#include <dryos.h>
#include <module.h>
#include <menu.h>
#include <config.h>
#include <bmp.h>
#include <console.h>
#include <stdint.h>

unsigned int mlrust_deinit();
void mlrust_task();
unsigned int mlrust_init();

uint32_t rFONT_MED = FONT_MED;

struct menu_entry mlrust_menu[] =
{
    {
        .name = "mlrust World",
        .select = run_in_separate_task,
        .priv = mlrust_task,
        .help = "My Demo",
    },
};

void _msleep(uint32_t ms) {
	msleep(ms);
}

void add_menu() {
	menu_add("Debug", mlrust_menu, COUNT(mlrust_menu));
}

MODULE_INFO_START()
MODULE_INIT(mlrust_init)
MODULE_DEINIT(mlrust_deinit)
MODULE_INFO_END()

