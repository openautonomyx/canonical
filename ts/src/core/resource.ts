// Canonical Core — resource addressing.
//
// A resource is named "<ownerId>:<path>". The owner is the identity to the left
// of the first colon (identity ids are base64url and contain no colon). Paths
// may end in "/*" to denote a subtree; "*" denotes everything.

export function makeResource(ownerId: string, path: string): string {
  return ownerId + ":" + path;
}

export function resourceOwner(resource: string): string {
  const i = resource.indexOf(":");
  return i < 0 ? resource : resource.slice(0, i);
}

export function resourcePath(resource: string): string {
  const i = resource.indexOf(":");
  return i < 0 ? "" : resource.slice(i + 1);
}

export function pathSubsumes(parent: string, child: string): boolean {
  if (parent === child) return true;
  if (parent === "*") return true;
  if (parent.endsWith("/*")) return child.startsWith(parent.slice(0, -1));
  return false;
}

/** True when `parent` covers `child`: same owner and the path subsumes. */
export function resourceSubsumes(parent: string, child: string): boolean {
  return resourceOwner(parent) === resourceOwner(child) && pathSubsumes(resourcePath(parent), resourcePath(child));
}

export function actionSubsumes(parent: string, child: string): boolean {
  return parent === "*" || parent === child;
}
