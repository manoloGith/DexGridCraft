package io.github.dexgridcraft

import io.github.craftandroid.CraftActivity

/**
 * DexGridCraft's activity. Everything Android-side (documents through the Storage Access
 * Framework, clipboard, printing, mouse and keyboard, the on-screen keyboard, lifecycle,
 * diagnostics) is craft-android's [CraftActivity]; the library it loads (`libdexgridcraft.so`,
 * the manifest's `android.app.lib_name`) runs GridCraft's interface.
 */
class MainActivity : CraftActivity() {
    /** Workbooks dragged onto the window open as GridCraft's File › Open opens them. */
    override fun droppableMimeTypes(): List<String> = SPREADSHEET_TYPES

    companion object {
        /** What GridCraft opens: Excel (xlsx, xlsm, xls, xlsb), OpenDocument and CSV/TSV. */
        val SPREADSHEET_TYPES = listOf(
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "application/vnd.ms-excel.sheet.macroEnabled.12",
            "application/vnd.ms-excel",
            "application/vnd.ms-excel.sheet.binary.macroEnabled.12",
            "application/vnd.oasis.opendocument.spreadsheet",
            "text/csv",
            "text/comma-separated-values",
            "text/tab-separated-values",
        )
    }
}
