    #[zome_fn("hc_public")]
    fn delete_task(base: String, id: Address, created_at: Iso8601, address: Address) -> ZomeApiResult<Address> {
        task::handlers::delete(base, id, created_at, address)
    }