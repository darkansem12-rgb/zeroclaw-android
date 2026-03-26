package com.zeroclaw.app

import android.annotation.SuppressLint
import android.content.Context
import android.graphics.Bitmap
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.view.View
import android.view.WindowManager
import android.webkit.WebChromeClient
import android.webkit.WebResourceError
import android.webkit.WebResourceRequest
import android.webkit.WebSettings
import android.webkit.WebView
import android.webkit.WebViewClient
import android.widget.Button
import android.widget.ProgressBar
import android.widget.TextView
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import java.net.InetAddress
import java.net.Socket
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class MainActivity : AppCompatActivity() {

    companion object {
        private const val ZEROCLAW_HOST = "127.0.0.1"
        private const val ZEROCLAW_PORT = 42617
        private const val CONNECTION_TIMEOUT_MS = 5000
        private const val WEBVIEW_URL = "http://127.0.0.1:42617"
        private const val CONNECTION_CHECK_RETRIES = 3
        private const val RETRY_DELAY_MS = 2000L
    }

    private lateinit var webView: WebView
    private lateinit var progressBar: ProgressBar
    private lateinit var errorContainer: View
    private lateinit var errorText: TextView
    private lateinit var retryButton: Button
    private lateinit var statusText: TextView
    private val mainHandler = Handler(Looper.getMainLooper())

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        setupFullScreen()
        setContentView(R.layout.activity_main)
        
        initViews()
        setupWebView()
        
        checkAndConnect()
    }

    private fun setupFullScreen() {
        WindowCompat.setDecorFitsSystemWindows(window, false)
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        
        WindowInsetsControllerCompat(window, window.decorView).let { controller ->
            controller.hide(WindowInsetsCompat.Type.systemBars())
            controller.systemBarsBehavior = WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
        }
    }

    private fun initViews() {
        webView = findViewById(R.id.webView)
        progressBar = findViewById(R.id.progressBar)
        errorContainer = findViewById(R.id.errorContainer)
        errorText = findViewById(R.id.errorText)
        retryButton = findViewById(R.id.retryButton)
        statusText = findViewById(R.id.statusText)
        
        retryButton.setOnClickListener {
            checkAndConnect()
        }
    }

    @SuppressLint("SetJavaScriptEnabled")
    private fun setupWebView() {
        webView.settings.apply {
            javaScriptEnabled = true
            domStorageEnabled = true
            databaseEnabled = true
            cacheMode = WebSettings.LOAD_NO_CACHE
            useWideViewPort = true
            loadWithOverviewMode = true
            builtInZoomControls = true
            displayZoomControls = false
            setSupportZoom(true)
            mixedContentMode = WebSettings.MIXED_CONTENT_ALWAYS_ALLOW
            allowFileAccess = false
            allowContentAccess = false
        }

        webView.webViewClient = object : WebViewClient() {
            override fun onPageStarted(view: WebView?, url: String?, favicon: Bitmap?) {
                super.onPageStarted(view, url, favicon)
                showLoading()
            }

            override fun onPageFinished(view: WebView?, url: String?) {
                super.onPageFinished(view, url)
                showWebView()
            }

            override fun onReceivedError(
                view: WebView?,
                request: WebResourceRequest?,
                error: WebResourceError?
            ) {
                super.onReceivedError(view, request, error)
                if (request?.isForMainFrame == true) {
                    showError("Error al cargar la página.\n¿ZeroClaw está corriendo en Termux?")
                }
            }
        }

        webView.webChromeClient = object : WebChromeClient() {
            override fun onProgressChanged(view: WebView?, newProgress: Int) {
                super.onProgressChanged(view, newProgress)
                progressBar.progress = newProgress
            }
        }
    }

    private fun checkAndConnect() {
        showLoading()
        statusText.text = "Verificando conexión..."
        
        CoroutineScope(Dispatchers.Main).launch {
            val isConnected = checkZeroClawConnection()
            
            if (isConnected) {
                statusText.text = "Conectando a ZeroClaw..."
                loadWebView()
            } else {
                showError("No se puede conectar a ZeroClaw.\n\nAsegúrate de que ZeroClaw esté corriendo en Termux:\n1. Abre Termux\n2. Ejecuta: ./zeroclaw daemon\n3. Espera a que muestre 'Listening on http://127.0.0.1:42617'\n4. Regresa a esta app")
            }
        }
    }

    private suspend fun checkZeroClawConnection(): Boolean {
        return withContext(Dispatchers.IO) {
            repeat(CONNECTION_CHECK_RETRIES) { attempt ->
                try {
                    val address = InetAddress.getByName(ZEROCLAW_HOST)
                    if (address.isReachable(CONNECTION_TIMEOUT_MS)) {
                        return@withContext true
                    }
                } catch (e: Exception) {
                    // Continuar al siguiente intento
                }
                
                if (attempt < CONNECTION_CHECK_RETRIES - 1) {
                    Thread.sleep(RETRY_DELAY_MS)
                }
            }
            
            // Intentar conexión directa al puerto
            try {
                val socket = Socket(ZEROCLAW_HOST, ZEROCLAW_PORT)
                socket.close()
                return@withContext true
            } catch (e: Exception) {
                return@withContext false
            }
        }
    }

    private fun loadWebView() {
        webView.loadUrl(WEBVIEW_URL)
    }

    private fun showLoading() {
        progressBar.visibility = View.VISIBLE
        webView.visibility = View.GONE
        errorContainer.visibility = View.GONE
    }

    private fun showWebView() {
        progressBar.visibility = View.GONE
        webView.visibility = View.VISIBLE
        errorContainer.visibility = View.GONE
    }

    private fun showError(message: String) {
        progressBar.visibility = View.GONE
        webView.visibility = View.GONE
        errorContainer.visibility = View.VISIBLE
        errorText.text = message
    }

    override fun onBackPressed() {
        if (webView.canGoBack()) {
            webView.goBack()
        } else {
            super.onBackPressed()
        }
    }

    override fun onDestroy() {
        webView.destroy()
        super.onDestroy()
    }
}
