import { AdminApi, PublicApi } from "@oryd/kratos-client";

export const adminApi = new AdminApi("http://kratos-admin.kratos");

export const publicApi = new PublicApi("http://kratos-public.kratos");
