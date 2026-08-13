using System.Diagnostics;
using System.Drawing;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Windows.Forms;

namespace Heartlight.Windows;

internal static class Program
{
    [STAThread]
    private static void Main()
    {
        ApplicationConfiguration.Initialize();
        var root = Path.Combine(AppContext.BaseDirectory, "wwwroot");
        if (!Directory.Exists(root))
        {
            MessageBox.Show("HEARTLIGHT application files are missing.", "COSMOS HEARTLIGHT", MessageBoxButtons.OK, MessageBoxIcon.Error);
            return;
        }

        using var server = new LocalServer(root);
        server.Start();

        var menu = new ContextMenuStrip();
        menu.Items.Add("Open HEARTLIGHT", null, (_, _) => Open(server.Url));
        menu.Items.Add("Exit", null, (_, _) => Application.Exit());

        using var tray = new NotifyIcon
        {
            Text = "COSMOS HEARTLIGHT",
            Icon = SystemIcons.Application,
            Visible = true,
            ContextMenuStrip = menu
        };
        tray.DoubleClick += (_, _) => Open(server.Url);

        Open(server.Url);
        Application.Run();
        tray.Visible = false;
    }

    private static void Open(string url)
    {
        try { Process.Start(new ProcessStartInfo(url) { UseShellExecute = true }); }
        catch { MessageBox.Show(url, "Open HEARTLIGHT in your browser"); }
    }
}

internal sealed class LocalServer : IDisposable
{
    private readonly string _root;
    private readonly HttpListener _listener = new();
    private CancellationTokenSource? _cts;
    public string Url { get; private set; } = string.Empty;

    public LocalServer(string root) => _root = Path.GetFullPath(root);

    public void Start()
    {
        var port = FindPort();
        Url = $"http://127.0.0.1:{port}/";
        _listener.Prefixes.Add(Url);
        _listener.Start();
        _cts = new CancellationTokenSource();
        _ = Task.Run(() => Loop(_cts.Token));
    }

    private static int FindPort()
    {
        for (var port = 8877; port <= 8897; port++)
        {
            try
            {
                var tcp = new TcpListener(IPAddress.Loopback, port);
                tcp.Start(); tcp.Stop(); return port;
            }
            catch (SocketException) { }
        }
        throw new InvalidOperationException("No local HEARTLIGHT port is available.");
    }

    private async Task Loop(CancellationToken token)
    {
        while (!token.IsCancellationRequested && _listener.IsListening)
        {
            try
            {
                var context = await _listener.GetContextAsync();
                _ = Task.Run(() => Serve(context), token);
            }
            catch when (token.IsCancellationRequested) { break; }
            catch { }
        }
    }

    private async Task Serve(HttpListenerContext context)
    {
        try
        {
            var requestPath = Uri.UnescapeDataString(context.Request.Url?.AbsolutePath ?? "/");
            requestPath = requestPath.TrimStart('/');
            if (string.IsNullOrWhiteSpace(requestPath)) requestPath = "index.html";

            var fullPath = Path.GetFullPath(Path.Combine(_root, requestPath.Replace('/', Path.DirectorySeparatorChar)));
            if (!fullPath.StartsWith(_root, StringComparison.OrdinalIgnoreCase) || !File.Exists(fullPath))
            {
                context.Response.StatusCode = 404;
                await WriteText(context.Response, "Not found");
                return;
            }

            var bytes = await File.ReadAllBytesAsync(fullPath);
            context.Response.ContentType = Mime(fullPath);
            context.Response.ContentLength64 = bytes.Length;
            context.Response.Headers["Cache-Control"] = "no-store";
            await context.Response.OutputStream.WriteAsync(bytes);
            context.Response.OutputStream.Close();
        }
        catch
        {
            try { context.Response.StatusCode = 500; context.Response.Close(); } catch { }
        }
    }

    private static async Task WriteText(HttpListenerResponse response, string text)
    {
        var bytes = Encoding.UTF8.GetBytes(text);
        response.ContentType = "text/plain; charset=utf-8";
        response.ContentLength64 = bytes.Length;
        await response.OutputStream.WriteAsync(bytes);
        response.OutputStream.Close();
    }

    private static string Mime(string path) => Path.GetExtension(path).ToLowerInvariant() switch
    {
        ".html" => "text/html; charset=utf-8",
        ".js" => "text/javascript; charset=utf-8",
        ".css" => "text/css; charset=utf-8",
        ".json" or ".webmanifest" => "application/json; charset=utf-8",
        ".svg" => "image/svg+xml",
        ".png" => "image/png",
        ".ico" => "image/x-icon",
        _ => "application/octet-stream"
    };

    public void Dispose()
    {
        try { _cts?.Cancel(); } catch { }
        try { _listener.Stop(); _listener.Close(); } catch { }
        _cts?.Dispose();
    }
}
