use serde_json::Value;
use tower_lsp::jsonrpc::{Error, Result};
use tower_lsp::lsp_types::request::*;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

#[derive(Debug)]
struct Backend {
    client: Client,
    log_file: Mutex<File>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        writeln!(self.log_file.lock().unwrap(), "initialize").unwrap();

        let mut result = InitializeResult::default();
        let mut server_caps = ServerCapabilities::default();
        server_caps.declaration_provider = Some(DeclarationCapability::Simple(true));
        server_caps.text_document_sync = Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                will_save: None,
                will_save_wait_until: None,
                save: Some(SaveOptions::default().into()),
            },
        ));
        result.capabilities = server_caps;

        Ok(result)
    }

    async fn initialized(&self, _: InitializedParams) {
        writeln!(self.log_file.lock().unwrap(), "initialized").unwrap();
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        writeln!(self.log_file.lock().unwrap(), "shutdown").unwrap();
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn will_save(&self, params: WillSaveTextDocumentParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn will_save_wait_until(
        &self,
        params: WillSaveTextDocumentParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn goto_declaration(
        &self,
        params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn goto_type_definition(
        &self,
        params: GotoTypeDefinitionParams,
    ) -> Result<Option<GotoTypeDefinitionResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn goto_implementation(
        &self,
        params: GotoImplementationParams,
    ) -> Result<Option<GotoImplementationResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn references(&self, params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn prepare_call_hierarchy(
        &self,
        params: CallHierarchyPrepareParams,
    ) -> Result<Option<Vec<CallHierarchyItem>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn incoming_calls(
        &self,
        params: CallHierarchyIncomingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn outgoing_calls(
        &self,
        params: CallHierarchyOutgoingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn prepare_type_hierarchy(
        &self,
        params: TypeHierarchyPrepareParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn supertypes(
        &self,
        params: TypeHierarchySupertypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn subtypes(
        &self,
        params: TypeHierarchySubtypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn document_highlight(
        &self,
        params: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn document_link(&self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn document_link_resolve(&self, params: DocumentLink) -> Result<DocumentLink> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn code_lens_resolve(&self, params: CodeLens) -> Result<CodeLens> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn folding_range(&self, params: FoldingRangeParams) -> Result<Option<Vec<FoldingRange>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn selection_range(
        &self,
        params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn semantic_tokens_full_delta(
        &self,
        params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn semantic_tokens_range(
        &self,
        params: SemanticTokensRangeParams,
    ) -> Result<Option<SemanticTokensRangeResult>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn inline_value(&self, params: InlineValueParams) -> Result<Option<Vec<InlineValue>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> Result<Option<Vec<InlayHint>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn inlay_hint_resolve(&self, params: InlayHint) -> Result<InlayHint> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn moniker(&self, params: MonikerParams) -> Result<Option<Vec<Moniker>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn completion_resolve(&self, params: CompletionItem) -> Result<CompletionItem> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn workspace_diagnostic(
        &self,
        params: WorkspaceDiagnosticParams,
    ) -> Result<WorkspaceDiagnosticReportResult> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn code_action_resolve(&self, params: CodeAction) -> Result<CodeAction> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn color_presentation(
        &self,
        params: ColorPresentationParams,
    ) -> Result<Vec<ColorPresentation>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn range_formatting(
        &self,
        params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn on_type_formatting(
        &self,
        params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn linked_editing_range(
        &self,
        params: LinkedEditingRangeParams,
    ) -> Result<Option<LinkedEditingRanges>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn symbol_resolve(&self, params: WorkspaceSymbol) -> Result<WorkspaceSymbol> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn did_change_workspace_folders(&self, params: DidChangeWorkspaceFoldersParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn will_create_files(&self, params: CreateFilesParams) -> Result<Option<WorkspaceEdit>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn did_create_files(&self, params: CreateFilesParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn will_rename_files(&self, params: RenameFilesParams) -> Result<Option<WorkspaceEdit>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn did_rename_files(&self, params: RenameFilesParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn will_delete_files(&self, params: DeleteFilesParams) -> Result<Option<WorkspaceEdit>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }

    async fn did_delete_files(&self, params: DeleteFilesParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
    }

    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        writeln!(self.log_file.lock().unwrap(), "{:#?}", params).unwrap();
        Err(Error::method_not_found())
    }
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let log_file = File::create("/home/omer/lsp_test_logs").unwrap();

        let (service, socket) = LspService::new(|client| Backend {
            client,
            log_file: Mutex::new(log_file),
        });
        Server::new(stdin, stdout, socket).serve(service).await;
    });
}
