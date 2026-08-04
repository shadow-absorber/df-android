package itmr.dragonfable

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import itmr.dragonfable.ui.theme.RuffleTheme

class PanicActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        enableEdgeToEdge()
        super.onCreate(savedInstanceState)

        setContent {
            RuffleTheme {
                PanicScreen(message = intent.getStringExtra("message") ?: "Unknown")
            }
        }
    }
}
