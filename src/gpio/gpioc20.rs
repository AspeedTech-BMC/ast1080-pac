#[doc = "Register `GPIOC20` reader"]
pub type R = crate::R<Gpioc20Spec>;
#[doc = "Register `GPIOC20` writer"]
pub type W = crate::W<Gpioc20Spec>;
#[doc = "GPIO128 Read Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio128readPrivilegeRstTolerance {
    #[doc = "0: Read Privilege of GPIO128 is reset by WDT."]
    ReadPrivilegeOfGpio128IsResetByWdt = 0,
    #[doc = "1: Read Privilege of GPIO128 is NOT reset by WDT."]
    ReadPrivilegeOfGpio128IsNotResetByWdt = 1,
}
impl From<Gpio128readPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio128readPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO128ReadPrivilegeRstTolerance` reader - GPIO128 Read Privilege Reset Tolerance"]
pub type Gpio128readPrivilegeRstToleranceR = crate::BitReader<Gpio128readPrivilegeRstTolerance>;
impl Gpio128readPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio128readPrivilegeRstTolerance {
        match self.bits {
            false => Gpio128readPrivilegeRstTolerance::ReadPrivilegeOfGpio128IsResetByWdt,
            true => Gpio128readPrivilegeRstTolerance::ReadPrivilegeOfGpio128IsNotResetByWdt,
        }
    }
    #[doc = "Read Privilege of GPIO128 is reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio128_is_reset_by_wdt(&self) -> bool {
        *self == Gpio128readPrivilegeRstTolerance::ReadPrivilegeOfGpio128IsResetByWdt
    }
    #[doc = "Read Privilege of GPIO128 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio128_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio128readPrivilegeRstTolerance::ReadPrivilegeOfGpio128IsNotResetByWdt
    }
}
#[doc = "Field `GPIO128ReadPrivilegeRstTolerance` writer - GPIO128 Read Privilege Reset Tolerance"]
pub type Gpio128readPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio128readPrivilegeRstTolerance>;
impl<'a, REG> Gpio128readPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Privilege of GPIO128 is reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio128_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio128readPrivilegeRstTolerance::ReadPrivilegeOfGpio128IsResetByWdt)
    }
    #[doc = "Read Privilege of GPIO128 is NOT reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio128_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio128readPrivilegeRstTolerance::ReadPrivilegeOfGpio128IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO129ReadPrivilegeRstTolerance` reader - GPIO129 Read Privilege Reset Tolerance"]
pub type Gpio129readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO129ReadPrivilegeRstTolerance` writer - GPIO129 Read Privilege Reset Tolerance"]
pub type Gpio129readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO130ReadPrivilegeRstTolerance` reader - GPIO130 Read Privilege Reset Tolerance"]
pub type Gpio130readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO130ReadPrivilegeRstTolerance` writer - GPIO130 Read Privilege Reset Tolerance"]
pub type Gpio130readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO131ReadPrivilegeRstTolerance` reader - GPIO131 Read Privilege Reset Tolerance"]
pub type Gpio131readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO131ReadPrivilegeRstTolerance` writer - GPIO131 Read Privilege Reset Tolerance"]
pub type Gpio131readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO132ReadPrivilegeRstTolerance` reader - GPIO132 Read Privilege Reset Tolerance"]
pub type Gpio132readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO132ReadPrivilegeRstTolerance` writer - GPIO132 Read Privilege Reset Tolerance"]
pub type Gpio132readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO133ReadPrivilegeRstTolerance` reader - GPIO133 Read Privilege Reset Tolerance"]
pub type Gpio133readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO133ReadPrivilegeRstTolerance` writer - GPIO133 Read Privilege Reset Tolerance"]
pub type Gpio133readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO134ReadPrivilegeRstTolerance` reader - GPIO134 Read Privilege Reset Tolerance"]
pub type Gpio134readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO134ReadPrivilegeRstTolerance` writer - GPIO134 Read Privilege Reset Tolerance"]
pub type Gpio134readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO135ReadPrivilegeRstTolerance` reader - GPIO135 Read Privilege Reset Tolerance"]
pub type Gpio135readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO135ReadPrivilegeRstTolerance` writer - GPIO135 Read Privilege Reset Tolerance"]
pub type Gpio135readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO136ReadPrivilegeRstTolerance` reader - GPIO136 Read Privilege Reset Tolerance"]
pub type Gpio136readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO136ReadPrivilegeRstTolerance` writer - GPIO136 Read Privilege Reset Tolerance"]
pub type Gpio136readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO137ReadPrivilegeRstTolerance` reader - GPIO137 Read Privilege Reset Tolerance"]
pub type Gpio137readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO137ReadPrivilegeRstTolerance` writer - GPIO137 Read Privilege Reset Tolerance"]
pub type Gpio137readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO138ReadPrivilegeRstTolerance` reader - GPIO138 Read Privilege Reset Tolerance"]
pub type Gpio138readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO138ReadPrivilegeRstTolerance` writer - GPIO138 Read Privilege Reset Tolerance"]
pub type Gpio138readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO139ReadPrivilegeRstTolerance` reader - GPIO139 Read Privilege Reset Tolerance"]
pub type Gpio139readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO139ReadPrivilegeRstTolerance` writer - GPIO139 Read Privilege Reset Tolerance"]
pub type Gpio139readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO140ReadPrivilegeRstTolerance` reader - GPIO140 Read Privilege Reset Tolerance"]
pub type Gpio140readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO140ReadPrivilegeRstTolerance` writer - GPIO140 Read Privilege Reset Tolerance"]
pub type Gpio140readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO141ReadPrivilegeRstTolerance` reader - GPIO141 Read Privilege Reset Tolerance"]
pub type Gpio141readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO141ReadPrivilegeRstTolerance` writer - GPIO141 Read Privilege Reset Tolerance"]
pub type Gpio141readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO142ReadPrivilegeRstTolerance` reader - GPIO142 Read Privilege Reset Tolerance"]
pub type Gpio142readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO142ReadPrivilegeRstTolerance` writer - GPIO142 Read Privilege Reset Tolerance"]
pub type Gpio142readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO143ReadPrivilegeRstTolerance` reader - GPIO143 Read Privilege Reset Tolerance"]
pub type Gpio143readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO143ReadPrivilegeRstTolerance` writer - GPIO143 Read Privilege Reset Tolerance"]
pub type Gpio143readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO144ReadPrivilegeRstTolerance` reader - GPIO144 Read Privilege Reset Tolerance"]
pub type Gpio144readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO144ReadPrivilegeRstTolerance` writer - GPIO144 Read Privilege Reset Tolerance"]
pub type Gpio144readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO145ReadPrivilegeRstTolerance` reader - GPIO145 Read Privilege Reset Tolerance"]
pub type Gpio145readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO145ReadPrivilegeRstTolerance` writer - GPIO145 Read Privilege Reset Tolerance"]
pub type Gpio145readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO146ReadPrivilegeRstTolerance` reader - GPIO146 Read Privilege Reset Tolerance"]
pub type Gpio146readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO146ReadPrivilegeRstTolerance` writer - GPIO146 Read Privilege Reset Tolerance"]
pub type Gpio146readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO147ReadPrivilegeRstTolerance` reader - GPIO147 Read Privilege Reset Tolerance"]
pub type Gpio147readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO147ReadPrivilegeRstTolerance` writer - GPIO147 Read Privilege Reset Tolerance"]
pub type Gpio147readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO148ReadPrivilegeRstTolerance` reader - GPIO148 Read Privilege Reset Tolerance"]
pub type Gpio148readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO148ReadPrivilegeRstTolerance` writer - GPIO148 Read Privilege Reset Tolerance"]
pub type Gpio148readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO149ReadPrivilegeRstTolerance` reader - GPIO149 Read Privilege Reset Tolerance"]
pub type Gpio149readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO149ReadPrivilegeRstTolerance` writer - GPIO149 Read Privilege Reset Tolerance"]
pub type Gpio149readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO150ReadPrivilegeRstTolerance` reader - GPIO150 Read Privilege Reset Tolerance"]
pub type Gpio150readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO150ReadPrivilegeRstTolerance` writer - GPIO150 Read Privilege Reset Tolerance"]
pub type Gpio150readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO151ReadPrivilegeRstTolerance` reader - GPIO151 Read Privilege Reset Tolerance"]
pub type Gpio151readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO151ReadPrivilegeRstTolerance` writer - GPIO151 Read Privilege Reset Tolerance"]
pub type Gpio151readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO152ReadPrivilegeRstTolerance` reader - GPIO152 Read Privilege Reset Tolerance"]
pub type Gpio152readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO152ReadPrivilegeRstTolerance` writer - GPIO152 Read Privilege Reset Tolerance"]
pub type Gpio152readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO153ReadPrivilegeRstTolerance` reader - GPIO153 Read Privilege Reset Tolerance"]
pub type Gpio153readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO153ReadPrivilegeRstTolerance` writer - GPIO153 Read Privilege Reset Tolerance"]
pub type Gpio153readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO154ReadPrivilegeRstTolerance` reader - GPIO154 Read Privilege Reset Tolerance"]
pub type Gpio154readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO154ReadPrivilegeRstTolerance` writer - GPIO154 Read Privilege Reset Tolerance"]
pub type Gpio154readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO155ReadPrivilegeRstTolerance` reader - GPIO155 Read Privilege Reset Tolerance"]
pub type Gpio155readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO155ReadPrivilegeRstTolerance` writer - GPIO155 Read Privilege Reset Tolerance"]
pub type Gpio155readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO156ReadPrivilegeRstTolerance` reader - GPIO156 Read Privilege Reset Tolerance"]
pub type Gpio156readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO156ReadPrivilegeRstTolerance` writer - GPIO156 Read Privilege Reset Tolerance"]
pub type Gpio156readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO157ReadPrivilegeRstTolerance` reader - GPIO157 Read Privilege Reset Tolerance"]
pub type Gpio157readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO157ReadPrivilegeRstTolerance` writer - GPIO157 Read Privilege Reset Tolerance"]
pub type Gpio157readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO158ReadPrivilegeRstTolerance` reader - GPIO158 Read Privilege Reset Tolerance"]
pub type Gpio158readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO158ReadPrivilegeRstTolerance` writer - GPIO158 Read Privilege Reset Tolerance"]
pub type Gpio158readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO159ReadPrivilegeRstTolerance` reader - GPIO159 Read Privilege Reset Tolerance"]
pub type Gpio159readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO159ReadPrivilegeRstTolerance` writer - GPIO159 Read Privilege Reset Tolerance"]
pub type Gpio159readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO128 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio128read_privilege_rst_tolerance(&self) -> Gpio128readPrivilegeRstToleranceR {
        Gpio128readPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO129 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio129read_privilege_rst_tolerance(&self) -> Gpio129readPrivilegeRstToleranceR {
        Gpio129readPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO130 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio130read_privilege_rst_tolerance(&self) -> Gpio130readPrivilegeRstToleranceR {
        Gpio130readPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO131 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio131read_privilege_rst_tolerance(&self) -> Gpio131readPrivilegeRstToleranceR {
        Gpio131readPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO132 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio132read_privilege_rst_tolerance(&self) -> Gpio132readPrivilegeRstToleranceR {
        Gpio132readPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO133 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio133read_privilege_rst_tolerance(&self) -> Gpio133readPrivilegeRstToleranceR {
        Gpio133readPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO134 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio134read_privilege_rst_tolerance(&self) -> Gpio134readPrivilegeRstToleranceR {
        Gpio134readPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO135 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio135read_privilege_rst_tolerance(&self) -> Gpio135readPrivilegeRstToleranceR {
        Gpio135readPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO136 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio136read_privilege_rst_tolerance(&self) -> Gpio136readPrivilegeRstToleranceR {
        Gpio136readPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO137 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio137read_privilege_rst_tolerance(&self) -> Gpio137readPrivilegeRstToleranceR {
        Gpio137readPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO138 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio138read_privilege_rst_tolerance(&self) -> Gpio138readPrivilegeRstToleranceR {
        Gpio138readPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO139 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio139read_privilege_rst_tolerance(&self) -> Gpio139readPrivilegeRstToleranceR {
        Gpio139readPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO140 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio140read_privilege_rst_tolerance(&self) -> Gpio140readPrivilegeRstToleranceR {
        Gpio140readPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO141 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio141read_privilege_rst_tolerance(&self) -> Gpio141readPrivilegeRstToleranceR {
        Gpio141readPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO142 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio142read_privilege_rst_tolerance(&self) -> Gpio142readPrivilegeRstToleranceR {
        Gpio142readPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO143 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio143read_privilege_rst_tolerance(&self) -> Gpio143readPrivilegeRstToleranceR {
        Gpio143readPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO144 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio144read_privilege_rst_tolerance(&self) -> Gpio144readPrivilegeRstToleranceR {
        Gpio144readPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO145 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio145read_privilege_rst_tolerance(&self) -> Gpio145readPrivilegeRstToleranceR {
        Gpio145readPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO146 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio146read_privilege_rst_tolerance(&self) -> Gpio146readPrivilegeRstToleranceR {
        Gpio146readPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO147 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio147read_privilege_rst_tolerance(&self) -> Gpio147readPrivilegeRstToleranceR {
        Gpio147readPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO148 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio148read_privilege_rst_tolerance(&self) -> Gpio148readPrivilegeRstToleranceR {
        Gpio148readPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO149 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio149read_privilege_rst_tolerance(&self) -> Gpio149readPrivilegeRstToleranceR {
        Gpio149readPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO150 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio150read_privilege_rst_tolerance(&self) -> Gpio150readPrivilegeRstToleranceR {
        Gpio150readPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO151 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio151read_privilege_rst_tolerance(&self) -> Gpio151readPrivilegeRstToleranceR {
        Gpio151readPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO152 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio152read_privilege_rst_tolerance(&self) -> Gpio152readPrivilegeRstToleranceR {
        Gpio152readPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO153 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio153read_privilege_rst_tolerance(&self) -> Gpio153readPrivilegeRstToleranceR {
        Gpio153readPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO154 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio154read_privilege_rst_tolerance(&self) -> Gpio154readPrivilegeRstToleranceR {
        Gpio154readPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO155 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio155read_privilege_rst_tolerance(&self) -> Gpio155readPrivilegeRstToleranceR {
        Gpio155readPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO156 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio156read_privilege_rst_tolerance(&self) -> Gpio156readPrivilegeRstToleranceR {
        Gpio156readPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO157 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio157read_privilege_rst_tolerance(&self) -> Gpio157readPrivilegeRstToleranceR {
        Gpio157readPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO158 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio158read_privilege_rst_tolerance(&self) -> Gpio158readPrivilegeRstToleranceR {
        Gpio158readPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO159 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio159read_privilege_rst_tolerance(&self) -> Gpio159readPrivilegeRstToleranceR {
        Gpio159readPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO128 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio128read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio128readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio128readPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO129 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio129read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio129readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio129readPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO130 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio130read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio130readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio130readPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO131 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio131read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio131readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio131readPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO132 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio132read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio132readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio132readPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO133 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio133read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio133readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio133readPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO134 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio134read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio134readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio134readPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO135 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio135read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio135readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio135readPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO136 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio136read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio136readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio136readPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO137 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio137read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio137readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio137readPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO138 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio138read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio138readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio138readPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO139 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio139read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio139readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio139readPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO140 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio140read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio140readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio140readPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO141 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio141read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio141readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio141readPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO142 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio142read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio142readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio142readPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO143 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio143read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio143readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio143readPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO144 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio144read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio144readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio144readPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO145 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio145read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio145readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio145readPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO146 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio146read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio146readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio146readPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO147 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio147read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio147readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio147readPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO148 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio148read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio148readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio148readPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO149 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio149read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio149readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio149readPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO150 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio150read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio150readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio150readPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO151 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio151read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio151readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio151readPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO152 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio152read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio152readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio152readPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO153 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio153read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio153readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio153readPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO154 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio154read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio154readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio154readPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO155 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio155read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio155readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio155readPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO156 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio156read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio156readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio156readPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO157 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio157read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio157readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio157readPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO158 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio158read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio158readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio158readPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO159 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio159read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio159readPrivilegeRstToleranceW<Gpioc20Spec> {
        Gpio159readPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Read Privilege Reset Tolerance Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioc20Spec;
impl crate::RegisterSpec for Gpioc20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc20::R`](R) reader structure"]
impl crate::Readable for Gpioc20Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioc20::W`](W) writer structure"]
impl crate::Writable for Gpioc20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC20 to value 0"]
impl crate::Resettable for Gpioc20Spec {}
