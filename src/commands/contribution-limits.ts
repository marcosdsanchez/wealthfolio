import { getRunEnv, invokeTauri, invokeWeb, logger, RUN_ENV } from "@/adapters";
import { ContributionLimit, DepositsCalculation, NewContributionLimit } from "@/lib/types";

export const getContributionLimit = async (): Promise<ContributionLimit[]> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return invokeTauri("get_contribution_limits");
      case RUN_ENV.WEB:
        return invokeWeb("get_contribution_limits");
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error("Error fetching contribution limits.");
    throw error;
  }
};

export const createContributionLimit = async (
  newLimit: NewContributionLimit,
): Promise<ContributionLimit> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return invokeTauri("create_contribution_limit", { newLimit });
      case RUN_ENV.WEB:
        return invokeWeb("create_contribution_limit", { newLimit });
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error("Error creating contribution limit.");
    throw error;
  }
};

export const updateContributionLimit = async (
  id: string,
  updatedLimit: NewContributionLimit,
): Promise<ContributionLimit> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return invokeTauri("update_contribution_limit", { id, updatedLimit });
      case RUN_ENV.WEB:
        return invokeWeb("update_contribution_limit", { id, updatedLimit });
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error("Error updating contribution limit.");
    throw error;
  }
};

export const deleteContributionLimit = async (id: string): Promise<void> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return invokeTauri("delete_contribution_limit", { id });
      case RUN_ENV.WEB:
        return invokeWeb("delete_contribution_limit", { id });
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error("Error deleting contribution limit.");
    throw error;
  }
};

export const calculateDepositsForLimit = async (
  limitId: string,
  timezoneOffsetMinutes?: number,
): Promise<DepositsCalculation> => {
  try {
    switch (getRunEnv()) {
      case RUN_ENV.DESKTOP:
        return invokeTauri("calculate_deposits_for_contribution_limit", {
          limitId,
          timezone_offset_minutes: timezoneOffsetMinutes,
        });
      case RUN_ENV.WEB:
        return invokeWeb("calculate_deposits_for_contribution_limit", {
          limitId,
          timezoneOffsetMinutes,
        });
      default:
        throw new Error(`Unsupported`);
    }
  } catch (error) {
    logger.error("Error calculating deposits for contribution limit.");
    throw error;
  }
};
