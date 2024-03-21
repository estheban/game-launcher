
export interface Chunk {
    name: string;
    size: number;
    hash: string;
    file_permissions: number;
}

export interface Manifest {
    name: string;
    version: string;
    install_directory: string;
    files: Chunk[];
}

