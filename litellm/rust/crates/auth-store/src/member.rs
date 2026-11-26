//! A roster entry as the auth cache holds it, in a cached key and in a cached team's roster.

use litellm_redis_store::auth_cache::CachedMember;

use crate::Error;

/// The roster entry as `Member` dumps it: its three fields, whatever else the entry holds. The
/// model refuses an entry that names neither a user id nor an email.
pub(crate) fn member(member: CachedMember) -> Result<CachedMember, Error> {
    if member.user_id.is_none() && member.user_email.is_none() {
        return Err(Error::Row {
            column: "team_member",
        });
    }
    Ok(CachedMember {
        unread: Vec::new(),
        ..member
    })
}
