/** Port — agent patch accept/reject on disk. */
export interface IPatchRepository {
    acceptPatch(path: string): Promise<void>;
    rejectPatch(path: string): Promise<void>;
    revertContent(path: string, content: string): Promise<void>;
}
