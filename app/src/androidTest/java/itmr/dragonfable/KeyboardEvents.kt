package itmr.dragonfable

import android.content.ComponentName
import android.content.Context
import android.content.Intent
import android.graphics.Point
import android.graphics.Rect
import android.net.Uri
import android.os.SystemClock
import androidx.test.core.app.ApplicationProvider
import androidx.test.espresso.matcher.ViewMatchers
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.uiautomator.By
import androidx.test.uiautomator.UiDevice
import androidx.test.uiautomator.Until
import java.io.File
import java.util.concurrent.TimeoutException
import kotlin.math.min
import kotlin.math.roundToInt
import org.hamcrest.CoreMatchers
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

private const val BASIC_SAMPLE_PACKAGE = "itmr.dragonfable"
private const val LAUNCH_TIMEOUT = 5000L
private const val SWF_WIDTH = 800.0
private const val SWF_HEIGHT = 600.0

@RunWith(AndroidJUnit4::class)
class KeyboardEvents {
    private lateinit var device: UiDevice
    private lateinit var traceOutput: File
    private lateinit var swfFile: File

    @Before
    fun startMainActivityFromHomeScreen() {
        device = UiDevice.getInstance(InstrumentationRegistry.getInstrumentation())

        device.pressHome()

        val launcherPackage: String = device.launcherPackageName
        ViewMatchers.assertThat(launcherPackage, CoreMatchers.notNullValue())
        device.wait(
            Until.hasObject(By.pkg(launcherPackage).depth(0)),
            LAUNCH_TIMEOUT
        )

        val context = ApplicationProvider.getApplicationContext<Context>()
        traceOutput = File.createTempFile("trace", ".txt", context.cacheDir)
        swfFile = File.createTempFile("movie", ".swf", context.cacheDir)
        val resources = InstrumentationRegistry.getInstrumentation().context.resources
        val inStream = resources.openRawResource(
            itmr.dragonfable.test.R.raw.edittext
        )
        val bytes = inStream.readBytes()
        swfFile.writeBytes(bytes)
        val intent = context.packageManager.getLaunchIntentForPackage(
            BASIC_SAMPLE_PACKAGE
        )?.apply {
            component = ComponentName("itmr.dragonfable", "itmr.dragonfable.PlayerActivity")
            data = Uri.fromFile(swfFile)
            putExtra("traceOutput", traceOutput.absolutePath)
            addFlags(Intent.FLAG_ACTIVITY_CLEAR_TASK)
        }
        context.startActivity(intent)

        device.wait(
            Until.hasObject(By.pkg(BASIC_SAMPLE_PACKAGE).depth(0)),
            LAUNCH_TIMEOUT
        )
    }

    @Test
    fun keyboardShowsWhenTextFieldIsFocused() {
        val player = waitForPlayer()

        device.click(screenToSwf(player.visibleBounds, Point(400, 300)))

        waitUntilImeVisibility(true)
    }

    @Test
    fun keyboardHidesOnFocusLossAndPlayerRemainsResponsive() {
        val player = waitForPlayer()
        val fieldCenter = Point(400, 300)
        val outsideField = Point(50, 50)

        device.click(screenToSwf(player.visibleBounds, fieldCenter))
        waitUntilImeVisibility(true)

        device.click(screenToSwf(player.visibleBounds, outsideField))
        waitUntilImeVisibility(false)

        device.click(screenToSwf(player.visibleBounds, fieldCenter))
        waitUntilImeVisibility(true)
    }

    private fun waitForPlayer() = device.wait(
        Until.findObject(By.desc("Ruffle Player")),
        LAUNCH_TIMEOUT
    )?.also {
        Thread.sleep(2000)
    } ?: throw TimeoutException("Ruffle Player surface never appeared")

    private fun waitUntilImeVisibility(expectedVisible: Boolean, timeoutMillis: Long = 5000) {
        val timeoutAt = SystemClock.uptimeMillis() + timeoutMillis
        while (SystemClock.uptimeMillis() < timeoutAt) {
            if (isImeVisible() == expectedVisible) {
                return
            }
            Thread.sleep(100)
        }
        throw TimeoutException(
            "Soft keyboard did not become ${if (expectedVisible) "visible" else "hidden"} " +
                "within $timeoutMillis ms"
        )
    }

    private fun isImeVisible(): Boolean {
        val output = device.executeShellCommand("dumpsys input_method")
        return output.contains("mInputShown=true")
    }

    private fun screenToSwf(playerBounds: Rect, point: Point): Point {
        val stretchX = playerBounds.width() / SWF_WIDTH
        val stretchY = playerBounds.height() / SWF_HEIGHT
        val scaleFactor = min(stretchX, stretchY)
        val swfScreenWidth = SWF_WIDTH * scaleFactor
        val swfScreenHeight = SWF_HEIGHT * scaleFactor
        val swfOffsetX = (playerBounds.width() - swfScreenWidth) / 2
        val swfOffsetY = (playerBounds.height() - swfScreenHeight) / 2
        return Point(
            (playerBounds.left + swfOffsetX + point.x * scaleFactor).roundToInt(),
            (playerBounds.top + swfOffsetY + point.y * scaleFactor).roundToInt()
        )
    }
}

private fun UiDevice.click(point: Point) {
    this.click(point.x, point.y)
}
