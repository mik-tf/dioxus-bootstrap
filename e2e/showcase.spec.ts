import { test, expect, Page } from "@playwright/test";

// Helper: wait for WASM app to hydrate
async function waitForApp(page: Page) {
  await page.waitForSelector("h1", { timeout: 15_000 });
  await expect(page.locator("h1")).toContainText("dioxus-bootstrap Showcase");
}

// Helper: click a top-level tab by label
async function clickTab(page: Page, label: string) {
  await page.locator(".nav-tabs .nav-link", { hasText: label }).first().click();
  // Small wait for tab content to render
  await page.waitForTimeout(300);
}

// ---------------------------------------------------------------------------
// App loads
// ---------------------------------------------------------------------------
test.describe("App", () => {
  test("loads and shows title", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await expect(page.locator(".lead").first()).toContainText("zero JavaScript");
  });

  test("navbar is visible", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await expect(page.locator(".navbar")).toBeVisible();
  });

  test("navbar uses navbar-nav structure with separated links", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);

    const links = page.locator(".navbar .navbar-nav .nav-link");
    await expect(links).toHaveCount(2);
    await expect(links.nth(0)).toContainText("Showcase");
    await expect(links.nth(1)).toContainText("Docs");

    for (const link of await links.all()) {
      const padding = await link.evaluate((el) => {
        const style = window.getComputedStyle(el);
        return {
          left: Number.parseFloat(style.paddingLeft),
          right: Number.parseFloat(style.paddingRight),
        };
      });
      expect(padding.left).toBeGreaterThanOrEqual(8);
      expect(padding.right).toBeGreaterThanOrEqual(8);
    }
  });

  test("theme toggle works", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    const html = page.locator("html");
    // Default is dark
    await expect(html).toHaveAttribute("data-bs-theme", "dark");
    // Click theme toggle (button has aria-label "Switch to light mode" when dark)
    await page.locator("button[aria-label='Switch to light mode']").click();
    await expect(html).toHaveAttribute("data-bs-theme", "light");
  });

  test("all 8 tabs exist", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    const tabs = ["Basics", "Forms", "Data", "Interactive", "Overlays", "Media", "Navigation", "More"];
    for (const tab of tabs) {
      await expect(page.locator(".nav-tabs .nav-link", { hasText: tab }).first()).toBeVisible();
    }
  });
});

// ---------------------------------------------------------------------------
// Basics tab
// ---------------------------------------------------------------------------
test.describe("Basics tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Basics");
  });

  test("buttons render", async ({ page }) => {
    await expect(page.locator(".btn-primary").first()).toBeVisible();
    await expect(page.locator(".btn-secondary").first()).toBeVisible();
    await expect(page.locator(".btn-success").first()).toBeVisible();
  });

  test("button group renders", async ({ page }) => {
    await expect(page.locator(".btn-group").first()).toBeVisible();
  });

  test("cards render", async ({ page }) => {
    await expect(page.locator(".card").first()).toBeVisible();
    await expect(page.locator(".card-body").first()).toBeVisible();
  });

  test("alerts render", async ({ page }) => {
    await expect(page.locator(".alert").first()).toBeVisible();
  });

  test("badges render", async ({ page }) => {
    await expect(page.locator(".badge").first()).toBeVisible();
  });

  test("grid system renders", async ({ page }) => {
    await expect(page.locator(".row").first()).toBeVisible();
    await expect(page.locator("[class*='col']").first()).toBeVisible();
  });

  test("breadcrumb renders", async ({ page }) => {
    await expect(page.locator(".breadcrumb").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Forms tab
// ---------------------------------------------------------------------------
test.describe("Forms tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Forms");
  });

  test("text inputs render", async ({ page }) => {
    await expect(page.locator(".form-control").first()).toBeVisible();
  });

  test("select renders", async ({ page }) => {
    await expect(page.locator(".form-select").first()).toBeVisible();
  });

  test("checkboxes render", async ({ page }) => {
    await expect(page.locator(".form-check-input").first()).toBeVisible();
  });

  test("switch renders", async ({ page }) => {
    await expect(page.locator(".form-switch").first()).toBeVisible();
  });

  test("range renders", async ({ page }) => {
    await expect(page.locator(".form-range").first()).toBeVisible();
  });

  test("floating labels render", async ({ page }) => {
    await expect(page.locator(".form-floating").first()).toBeVisible();
  });

  test("input group renders", async ({ page }) => {
    await expect(page.locator(".input-group").first()).toBeVisible();
  });

  test("validation feedback renders", async ({ page }) => {
    const valid = page.locator(".is-valid, .valid-feedback");
    const invalid = page.locator(".is-invalid, .invalid-feedback");
    await expect(valid.first()).toBeVisible();
    await expect(invalid.first()).toBeVisible();
  });

  test("input can be typed into", async ({ page }) => {
    const input = page.locator("input.form-control").first();
    await input.fill("test@example.com");
    await expect(input).toHaveValue("test@example.com");
  });
});

// ---------------------------------------------------------------------------
// Data tab
// ---------------------------------------------------------------------------
test.describe("Data tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Data");
  });

  test("table renders with rows", async ({ page }) => {
    await expect(page.locator("table.table").first()).toBeVisible();
    const rows = page.locator("table.table tbody tr");
    expect(await rows.count()).toBeGreaterThan(0);
  });

  test("striped table renders", async ({ page }) => {
    await expect(page.locator("table.table-striped").first()).toBeVisible();
  });

  test("list group renders", async ({ page }) => {
    await expect(page.locator(".list-group").first()).toBeVisible();
    await expect(page.locator(".list-group-item").first()).toBeVisible();
  });

  test("progress bars render", async ({ page }) => {
    await expect(page.locator(".progress").first()).toBeVisible();
    await expect(page.locator(".progress-bar").first()).toBeVisible();
  });

  test("spinners render", async ({ page }) => {
    await expect(page.locator(".spinner-border, .spinner-grow").first()).toBeVisible();
  });

  test("pagination renders", async ({ page }) => {
    await expect(page.locator(".pagination").first()).toBeVisible();
    await expect(page.locator(".page-item").first()).toBeVisible();
  });

  test("pagination click changes active page", async ({ page }) => {
    const page4 = page.locator(".page-link", { hasText: "4" }).first();
    await page4.click();
    await expect(page.locator(".page-item.active .page-link", { hasText: "4" }).first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Interactive tab
// ---------------------------------------------------------------------------
test.describe("Interactive tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Interactive");
  });

  test("modal opens and closes", async ({ page }) => {
    // Find the button that opens a modal
    const openBtn = page.locator(".btn", { hasText: /open modal|launch/i }).first();
    await openBtn.click();
    await expect(page.locator(".modal.show")).toBeVisible();
    // Close via the X button or close button
    const closeBtn = page.locator(".modal.show .btn-close, .modal.show .btn", { hasText: /close/i }).first();
    await closeBtn.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".modal.show")).not.toBeVisible();
  });

  test("dropdown opens", async ({ page }) => {
    const dropBtn = page.locator(".dropdown .btn, .dropdown-toggle").first();
    await dropBtn.click();
    await expect(page.locator(".dropdown-menu.show").first()).toBeVisible();
  });

  test("collapse toggles", async ({ page }) => {
    // The collapse toggle button says "Show Content" / "Hide Content"
    const toggleBtn = page.locator(".btn", { hasText: /show content/i }).first();
    await toggleBtn.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".collapse.show").first()).toBeVisible();
  });

  test("accordion renders and toggles", async ({ page }) => {
    await expect(page.locator(".accordion").first()).toBeVisible();
    // Item #1 is open by default; click item #2 to open it
    const item2 = page.locator(".accordion-button").nth(1);
    await item2.click();
    await page.waitForTimeout(400);
    // Item #2 should now be open
    await expect(page.locator(".accordion-collapse.collapse.show").first()).toBeVisible();
  });

  test("tabs inside section work", async ({ page }) => {
    // The interactive section has its own nested tabs
    const innerTab = page.locator(".tab-pane .nav-link, .card .nav-link").first();
    if (await innerTab.isVisible()) {
      await innerTab.click();
      await expect(innerTab).toHaveClass(/active/);
    }
  });

  test("toast renders", async ({ page }) => {
    const toastBtn = page.locator(".btn", { hasText: /toast/i }).first();
    await toastBtn.click();
    await expect(page.locator(".toast.show").first()).toBeVisible();
  });

  test("offcanvas opens and closes", async ({ page }) => {
    const offBtn = page.locator(".btn", { hasText: /offcanvas/i }).first();
    await offBtn.click();
    await expect(page.locator(".offcanvas.show").first()).toBeVisible();
    // Close
    const closeBtn = page.locator(".offcanvas.show .btn-close").first();
    await closeBtn.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".offcanvas.show")).not.toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Overlays tab
// ---------------------------------------------------------------------------
test.describe("Overlays tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Overlays");
  });

  test("tooltip hover shows role, aria, and placement", async ({ page }) => {
    const trigger = page.locator("#tooltip-hover-trigger");
    const wrapper = page.locator(".tooltip-wrapper", { has: trigger });

    await trigger.hover();

    const tooltip = page.locator(".tooltip.show", {
      hasText: "Tooltip on top",
    });
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toHaveAttribute("role", "tooltip");
    await expect(tooltip).toHaveClass(/bs-tooltip-top/);

    const tooltipId = await tooltip.getAttribute("id");
    expect(tooltipId).toBeTruthy();
    await expect(wrapper).toHaveAttribute("aria-describedby", tooltipId!);
  });

  test("tooltip opens from keyboard focus", async ({ page }) => {
    await page.locator("#tooltip-focus-trigger").focus();
    await expect(
      page.locator(".tooltip.show", { hasText: "Tooltip on focus" }),
    ).toBeVisible();
  });

  test("tooltip click trigger toggles", async ({ page }) => {
    const trigger = page.locator("#tooltip-click-trigger");
    const tooltip = page.locator(".tooltip.show", {
      hasText: "Tooltip on click",
    });

    await trigger.click();
    await expect(tooltip).toBeVisible();

    await trigger.click();
    await expect(tooltip).not.toBeVisible();
  });

  test("tooltip falls back when requested placement overflows viewport", async ({
    page,
  }) => {
    const trigger = page.locator("#tooltip-edge-trigger");
    await trigger.evaluate((element) => {
      const wrapper = element.closest(".tooltip-wrapper") as HTMLElement;
      wrapper.style.position = "fixed";
      wrapper.style.top = "2px";
      wrapper.style.left = "320px";
      wrapper.style.zIndex = "1100";
    });
    await page.waitForTimeout(100);

    await trigger.hover();

    const tooltip = page.locator(".tooltip.show", {
      hasText: "Fallback below when top overflows",
    });
    await expect(tooltip).toBeVisible();
    await expect(tooltip).toHaveClass(/bs-tooltip-bottom/);

    const box = await tooltip.boundingBox();
    expect(box?.y ?? -1).toBeGreaterThanOrEqual(0);
  });

  test("popover click shows role, aria, and default placement", async ({ page }) => {
    const trigger = page.locator("#popover-click-trigger");
    const wrapper = page.locator(".popover-wrapper", { has: trigger });

    await trigger.click();

    const popover = page.locator(".popover.show", {
      hasText: "Click Popover",
    });
    await expect(popover).toBeVisible();
    await expect(popover).toHaveAttribute("role", "tooltip");
    await expect(popover).toHaveClass(/bs-popover-end/);

    const popoverId = await popover.getAttribute("id");
    expect(popoverId).toBeTruthy();
    await expect(wrapper).toHaveAttribute("aria-describedby", popoverId!);
  });

  test("popover focus trigger dismisses on blur", async ({ page }) => {
    await page.locator("#popover-focus-trigger").focus();
    const popover = page.locator(".popover.show", {
      hasText: "Focus Dismiss",
    });
    await expect(popover).toBeVisible();

    await page.locator("#popover-click-trigger").focus();
    await expect(popover).not.toBeVisible();
  });

  test("popover outside click dismisses", async ({ page }) => {
    const trigger = page.locator("#popover-outside-trigger");
    const popover = page.locator(".popover.show", {
      hasText: "Outside Dismiss",
    });

    await trigger.click();
    await expect(popover).toBeVisible();

    await page.locator(".popover-backdrop").click({ position: { x: 5, y: 5 } });
    await expect(popover).not.toBeVisible();
  });

  test("popover falls back when requested placement overflows viewport", async ({
    page,
  }) => {
    const trigger = page.locator("#popover-edge-trigger");
    await page.locator("#popover-edge-container").evaluate((element) => {
      const wrapper = element as HTMLElement;
      wrapper.style.position = "fixed";
      wrapper.style.top = "2px";
      wrapper.style.left = "320px";
      wrapper.style.zIndex = "1100";
    });
    await page.waitForTimeout(100);

    await trigger.click();

    const popover = page.locator(".popover.show", {
      hasText: "Fallback Popover",
    });
    await expect(popover).toBeVisible();
    await expect(popover).toHaveClass(/bs-popover-bottom/);

    const box = await popover.boundingBox();
    expect(box?.y ?? -1).toBeGreaterThanOrEqual(0);
  });

  test("popover with empty title and body does not render", async ({ page }) => {
    await page.locator("#popover-empty-trigger").click();
    await expect(page.locator(".popover.show")).toHaveCount(0);
  });
});

// ---------------------------------------------------------------------------
// Media tab
// ---------------------------------------------------------------------------
test.describe("Media tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Media");
  });

  test("carousel renders with slides", async ({ page }) => {
    await expect(page.locator(".carousel").first()).toBeVisible();
    await expect(page.locator(".carousel-item.active").first()).toBeVisible();
  });

  test("carousel controls work", async ({ page }) => {
    const nextBtn = page.locator(".carousel-control-next").first();
    await nextBtn.click();
    await page.waitForTimeout(800);
    // After clicking next, the carousel should have transitioned
    await expect(page.locator(".carousel-item.active").first()).toBeVisible();
  });

  test("carousel indicators render", async ({ page }) => {
    await expect(page.locator(".carousel-indicators").first()).toBeVisible();
    const indicators = page.locator(".carousel-indicators button");
    expect(await indicators.count()).toBeGreaterThan(1);
  });

  test("figure renders", async ({ page }) => {
    await expect(page.locator("figure, .figure").first()).toBeVisible();
  });

  test("ratio/embed renders", async ({ page }) => {
    await expect(page.locator(".ratio").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Navigation tab
// ---------------------------------------------------------------------------
test.describe("Navigation tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "Navigation");
  });

  test("nav pills render", async ({ page }) => {
    await expect(page.locator(".nav-pills").first()).toBeVisible();
  });

  test("nav tabs render", async ({ page }) => {
    // The inner nav-tabs within the Navigation section
    const navTabs = page.locator(".tab-pane .nav-tabs, .tab-content .nav-tabs").first();
    await expect(navTabs).toBeVisible();
  });

  test("breadcrumb renders", async ({ page }) => {
    await expect(page.locator(".breadcrumb").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// More tab
// ---------------------------------------------------------------------------
test.describe("More tab", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await clickTab(page, "More");
  });

  test("placeholders render", async ({ page }) => {
    await expect(page.locator(".placeholder, .placeholder-glow, .placeholder-wave").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Responsive / Navbar
// ---------------------------------------------------------------------------
test.describe("Navbar", () => {
  test("navbar collapse toggles on small viewport", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 812 });
    await page.goto("/");
    await waitForApp(page);
    // Hamburger should be visible
    const toggler = page.locator(".navbar-toggler").first();
    await expect(toggler).toBeVisible();
    await toggler.click();
    await page.waitForTimeout(400);
    await expect(page.locator(".navbar-collapse.show, .navbar-collapse.collapsing").first()).toBeVisible();
  });
});

// ---------------------------------------------------------------------------
// Docs section
// ---------------------------------------------------------------------------
test.describe("Docs section", () => {
  test("component reference section exists", async ({ page }) => {
    await page.goto("/");
    await waitForApp(page);
    await expect(page.locator("#docs-section")).toBeVisible();
    await expect(page.locator("#docs-section h2")).toContainText("Component Reference");
  });
});
