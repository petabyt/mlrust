#include <stdio.h>
#include <dryos.h>
#include <module.h>
#include <menu.h>
#include <config.h>
#include <bmp.h>
#include <console.h>

void mlrust_task();

struct menu_entry mlrust_menu[] =
{
    {
        .name = "mlrust World",
        .select = run_in_separate_task,
        .priv = mlrust_task,
        .help = "My Demo",
    },
};

unsigned int mlrust_init()
{
    menu_add("Debug", mlrust_menu, COUNT(mlrust_menu));
    return 0;
}

unsigned int mlrust_deinit()
{
    return 0;
}

MODULE_INFO_START()
MODULE_INIT(mlrust_init)
MODULE_DEINIT(mlrust_deinit)
MODULE_INFO_END()

