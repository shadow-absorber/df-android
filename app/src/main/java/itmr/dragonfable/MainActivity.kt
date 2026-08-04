package itmr.dragonfable

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import androidx.activity.ComponentActivity

private const val DEFAULT_SWF_URL = "https://play.dragonfable.com/game/DFLoader.swf"

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        startActivity(
            Intent(this, PlayerActivity::class.java).apply {
                data = Uri.parse(DEFAULT_SWF_URL)
            }
        )
        finish()
    }
}
