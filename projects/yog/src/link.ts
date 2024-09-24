
enum Protocol {
    Https,
    Http,
    Wss,
    Ws,
}

class Link {
    protocol: Protocol;
    host: string;
    port: number;
    path: string;

    constructor(protocol: Protocol, host: string, port: number, path: string) {
        this.protocol = protocol;
        this.host = host;
        this.port = port;
        this.path = path;
    }

    static from(s: string): Link {
        const parts: string[] = s.split("://");
        const protocolString: string | undefined = parts[0];
        const protocol: Protocol = protocolStringToEnum(protocolString);
        const hostPortPath: string[] = parts[1].split("/");
        const hostPort: string = hostPortPath[0];
        const [host, portString] = hostPort.split(":");
        const port: number = portString ? parseInt(portString) : getPortForProtocol(protocol);
        const path: string = hostPortPath.slice(1).join("/");
        return new Link(protocol, host, port, path);
    }

    toString(): string {
        return `${protocolEnumToString(this.protocol)}://${this.host}:${this.port}/${this.path}`;
    }
}

function protocolStringToEnum(protocolString: string | undefined): Protocol {
    switch (protocolString) {
        case "https":
            return Protocol.Https;
        case "http":
            return Protocol.Http;
        case "wss":
            return Protocol.Wss;
        case "ws":
            return Protocol.Ws;
        default:
            return Protocol.Http; // Default to HTTP if protocol is not recognized
    }
}

function protocolEnumToString(protocol: Protocol): string {
    switch (protocol) {
        case Protocol.Https:
            return "https";
        case Protocol.Http:
            return "http";
        case Protocol.Wss:
            return "wss";
        case Protocol.Ws:
            return "ws";
    }
}

function getPortForProtocol(protocol: Protocol): number {
    switch (protocol) {
        case Protocol.Https:
            return 443;
        case Protocol.Http:
            return 80;
        case Protocol.Wss:
            return 443;
        case Protocol.Ws:
            return 80;
    }
}

function runPerformanceTest(linkString: string, iterations: number): void {
    let totalTime = 0;

    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        Link.from(linkString);
        const end = performance.now();
        totalTime += end - start;
    }

    console.log(`Total time for ${iterations} iterations: ${totalTime} milliseconds`);
    console.log(`Average time per iteration: ${totalTime / iterations} milliseconds`);
}

// Example usage:
const linkString = "https://example.com:8080/path/to/resource";
const iterations = 10000;
runPerformanceTest(linkString, iterations);
