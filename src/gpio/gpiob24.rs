#[doc = "Register `GPIOB24` reader"]
pub type R = crate::R<Gpiob24Spec>;
#[doc = "Register `GPIOB24` writer"]
pub type W = crate::W<Gpiob24Spec>;
#[doc = "GPIO160 Write Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio160wrPrivilegeRstTolerance {
    #[doc = "0: Write Privilege of GPIO160 is reset by WDT."]
    WritePrivilegeOfGpio160IsResetByWdt = 0,
    #[doc = "1: Write Privilege of GPIO160 is NOT reset by WDT."]
    WritePrivilegeOfGpio160IsNotResetByWdt = 1,
}
impl From<Gpio160wrPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio160wrPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO160WrPrivilegeRstTolerance` reader - GPIO160 Write Privilege Reset Tolerance"]
pub type Gpio160wrPrivilegeRstToleranceR = crate::BitReader<Gpio160wrPrivilegeRstTolerance>;
impl Gpio160wrPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio160wrPrivilegeRstTolerance {
        match self.bits {
            false => Gpio160wrPrivilegeRstTolerance::WritePrivilegeOfGpio160IsResetByWdt,
            true => Gpio160wrPrivilegeRstTolerance::WritePrivilegeOfGpio160IsNotResetByWdt,
        }
    }
    #[doc = "Write Privilege of GPIO160 is reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio160_is_reset_by_wdt(&self) -> bool {
        *self == Gpio160wrPrivilegeRstTolerance::WritePrivilegeOfGpio160IsResetByWdt
    }
    #[doc = "Write Privilege of GPIO160 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_write_privilege_of_gpio160_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio160wrPrivilegeRstTolerance::WritePrivilegeOfGpio160IsNotResetByWdt
    }
}
#[doc = "Field `GPIO160WrPrivilegeRstTolerance` writer - GPIO160 Write Privilege Reset Tolerance"]
pub type Gpio160wrPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio160wrPrivilegeRstTolerance>;
impl<'a, REG> Gpio160wrPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write Privilege of GPIO160 is reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio160_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio160wrPrivilegeRstTolerance::WritePrivilegeOfGpio160IsResetByWdt)
    }
    #[doc = "Write Privilege of GPIO160 is NOT reset by WDT."]
    #[inline(always)]
    pub fn write_privilege_of_gpio160_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio160wrPrivilegeRstTolerance::WritePrivilegeOfGpio160IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO161WrPrivilegeRstTolerance` reader - GPIO161 Write Privilege Reset Tolerance"]
pub type Gpio161wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO161WrPrivilegeRstTolerance` writer - GPIO161 Write Privilege Reset Tolerance"]
pub type Gpio161wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO162WrPrivilegeRstTolerance` reader - GPIO162 Write Privilege Reset Tolerance"]
pub type Gpio162wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO162WrPrivilegeRstTolerance` writer - GPIO162 Write Privilege Reset Tolerance"]
pub type Gpio162wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO163WrPrivilegeRstTolerance` reader - GPIO163 Write Privilege Reset Tolerance"]
pub type Gpio163wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO163WrPrivilegeRstTolerance` writer - GPIO163 Write Privilege Reset Tolerance"]
pub type Gpio163wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO164WrPrivilegeRstTolerance` reader - GPIO164 Write Privilege Reset Tolerance"]
pub type Gpio164wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO164WrPrivilegeRstTolerance` writer - GPIO164 Write Privilege Reset Tolerance"]
pub type Gpio164wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO165WrPrivilegeRstTolerance` reader - GPIO165 Write Privilege Reset Tolerance"]
pub type Gpio165wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO165WrPrivilegeRstTolerance` writer - GPIO165 Write Privilege Reset Tolerance"]
pub type Gpio165wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO166WrPrivilegeRstTolerance` reader - GPIO166 Write Privilege Reset Tolerance"]
pub type Gpio166wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO166WrPrivilegeRstTolerance` writer - GPIO166 Write Privilege Reset Tolerance"]
pub type Gpio166wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO167WrPrivilegeRstTolerance` reader - GPIO167 Write Privilege Reset Tolerance"]
pub type Gpio167wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO167WrPrivilegeRstTolerance` writer - GPIO167 Write Privilege Reset Tolerance"]
pub type Gpio167wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO168WrPrivilegeRstTolerance` reader - GPIO168 Write Privilege Reset Tolerance"]
pub type Gpio168wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO168WrPrivilegeRstTolerance` writer - GPIO168 Write Privilege Reset Tolerance"]
pub type Gpio168wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO169WrPrivilegeRstTolerance` reader - GPIO169 Write Privilege Reset Tolerance"]
pub type Gpio169wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO169WrPrivilegeRstTolerance` writer - GPIO169 Write Privilege Reset Tolerance"]
pub type Gpio169wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO170WrPrivilegeRstTolerance` reader - GPIO170 Write Privilege Reset Tolerance"]
pub type Gpio170wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO170WrPrivilegeRstTolerance` writer - GPIO170 Write Privilege Reset Tolerance"]
pub type Gpio170wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO171WrPrivilegeRstTolerance` reader - GPIO171 Write Privilege Reset Tolerance"]
pub type Gpio171wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO171WrPrivilegeRstTolerance` writer - GPIO171 Write Privilege Reset Tolerance"]
pub type Gpio171wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO172WrPrivilegeRstTolerance` reader - GPIO172 Write Privilege Reset Tolerance"]
pub type Gpio172wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO172WrPrivilegeRstTolerance` writer - GPIO172 Write Privilege Reset Tolerance"]
pub type Gpio172wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO173WrPrivilegeRstTolerance` reader - GPIO173 Write Privilege Reset Tolerance"]
pub type Gpio173wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO173WrPrivilegeRstTolerance` writer - GPIO173 Write Privilege Reset Tolerance"]
pub type Gpio173wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO174WrPrivilegeRstTolerance` reader - GPIO174 Write Privilege Reset Tolerance"]
pub type Gpio174wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO174WrPrivilegeRstTolerance` writer - GPIO174 Write Privilege Reset Tolerance"]
pub type Gpio174wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO175WrPrivilegeRstTolerance` reader - GPIO175 Write Privilege Reset Tolerance"]
pub type Gpio175wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO175WrPrivilegeRstTolerance` writer - GPIO175 Write Privilege Reset Tolerance"]
pub type Gpio175wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO176WrPrivilegeRstTolerance` reader - GPIO176 Write Privilege Reset Tolerance"]
pub type Gpio176wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO176WrPrivilegeRstTolerance` writer - GPIO176 Write Privilege Reset Tolerance"]
pub type Gpio176wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO177WrPrivilegeRstTolerance` reader - GPIO177 Write Privilege Reset Tolerance"]
pub type Gpio177wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO177WrPrivilegeRstTolerance` writer - GPIO177 Write Privilege Reset Tolerance"]
pub type Gpio177wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO178WrPrivilegeRstTolerance` reader - GPIO178 Write Privilege Reset Tolerance"]
pub type Gpio178wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO178WrPrivilegeRstTolerance` writer - GPIO178 Write Privilege Reset Tolerance"]
pub type Gpio178wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO179WrPrivilegeRstTolerance` reader - GPIO179 Write Privilege Reset Tolerance"]
pub type Gpio179wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO179WrPrivilegeRstTolerance` writer - GPIO179 Write Privilege Reset Tolerance"]
pub type Gpio179wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO180WrPrivilegeRstTolerance` reader - GPIO180 Write Privilege Reset Tolerance"]
pub type Gpio180wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO180WrPrivilegeRstTolerance` writer - GPIO180 Write Privilege Reset Tolerance"]
pub type Gpio180wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO181WrPrivilegeRstTolerance` reader - GPIO181 Write Privilege Reset Tolerance"]
pub type Gpio181wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO181WrPrivilegeRstTolerance` writer - GPIO181 Write Privilege Reset Tolerance"]
pub type Gpio181wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO182WrPrivilegeRstTolerance` reader - GPIO182 Write Privilege Reset Tolerance"]
pub type Gpio182wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO182WrPrivilegeRstTolerance` writer - GPIO182 Write Privilege Reset Tolerance"]
pub type Gpio182wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO183WrPrivilegeRstTolerance` reader - GPIO183 Write Privilege Reset Tolerance"]
pub type Gpio183wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO183WrPrivilegeRstTolerance` writer - GPIO183 Write Privilege Reset Tolerance"]
pub type Gpio183wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO184WrPrivilegeRstTolerance` reader - GPIO184 Write Privilege Reset Tolerance"]
pub type Gpio184wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO184WrPrivilegeRstTolerance` writer - GPIO184 Write Privilege Reset Tolerance"]
pub type Gpio184wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO185WrPrivilegeRstTolerance` reader - GPIO185 Write Privilege Reset Tolerance"]
pub type Gpio185wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO185WrPrivilegeRstTolerance` writer - GPIO185 Write Privilege Reset Tolerance"]
pub type Gpio185wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO186WrPrivilegeRstTolerance` reader - GPIO186 Write Privilege Reset Tolerance"]
pub type Gpio186wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO186WrPrivilegeRstTolerance` writer - GPIO186 Write Privilege Reset Tolerance"]
pub type Gpio186wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO187WrPrivilegeRstTolerance` reader - GPIO187 Write Privilege Reset Tolerance"]
pub type Gpio187wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO187WrPrivilegeRstTolerance` writer - GPIO187 Write Privilege Reset Tolerance"]
pub type Gpio187wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO188WrPrivilegeRstTolerance` reader - GPIO188 Write Privilege Reset Tolerance"]
pub type Gpio188wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO188WrPrivilegeRstTolerance` writer - GPIO188 Write Privilege Reset Tolerance"]
pub type Gpio188wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO189WrPrivilegeRstTolerance` reader - GPIO189 Write Privilege Reset Tolerance"]
pub type Gpio189wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO189WrPrivilegeRstTolerance` writer - GPIO189 Write Privilege Reset Tolerance"]
pub type Gpio189wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO190WrPrivilegeRstTolerance` reader - GPIO190 Write Privilege Reset Tolerance"]
pub type Gpio190wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO190WrPrivilegeRstTolerance` writer - GPIO190 Write Privilege Reset Tolerance"]
pub type Gpio190wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO191WrPrivilegeRstTolerance` reader - GPIO191 Write Privilege Reset Tolerance"]
pub type Gpio191wrPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO191WrPrivilegeRstTolerance` writer - GPIO191 Write Privilege Reset Tolerance"]
pub type Gpio191wrPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO160 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio160wr_privilege_rst_tolerance(&self) -> Gpio160wrPrivilegeRstToleranceR {
        Gpio160wrPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO161 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio161wr_privilege_rst_tolerance(&self) -> Gpio161wrPrivilegeRstToleranceR {
        Gpio161wrPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO162 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio162wr_privilege_rst_tolerance(&self) -> Gpio162wrPrivilegeRstToleranceR {
        Gpio162wrPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO163 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio163wr_privilege_rst_tolerance(&self) -> Gpio163wrPrivilegeRstToleranceR {
        Gpio163wrPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO164 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio164wr_privilege_rst_tolerance(&self) -> Gpio164wrPrivilegeRstToleranceR {
        Gpio164wrPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO165 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio165wr_privilege_rst_tolerance(&self) -> Gpio165wrPrivilegeRstToleranceR {
        Gpio165wrPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO166 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio166wr_privilege_rst_tolerance(&self) -> Gpio166wrPrivilegeRstToleranceR {
        Gpio166wrPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO167 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio167wr_privilege_rst_tolerance(&self) -> Gpio167wrPrivilegeRstToleranceR {
        Gpio167wrPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO168 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio168wr_privilege_rst_tolerance(&self) -> Gpio168wrPrivilegeRstToleranceR {
        Gpio168wrPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO169 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio169wr_privilege_rst_tolerance(&self) -> Gpio169wrPrivilegeRstToleranceR {
        Gpio169wrPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO170 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio170wr_privilege_rst_tolerance(&self) -> Gpio170wrPrivilegeRstToleranceR {
        Gpio170wrPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO171 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio171wr_privilege_rst_tolerance(&self) -> Gpio171wrPrivilegeRstToleranceR {
        Gpio171wrPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO172 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio172wr_privilege_rst_tolerance(&self) -> Gpio172wrPrivilegeRstToleranceR {
        Gpio172wrPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO173 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio173wr_privilege_rst_tolerance(&self) -> Gpio173wrPrivilegeRstToleranceR {
        Gpio173wrPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO174 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio174wr_privilege_rst_tolerance(&self) -> Gpio174wrPrivilegeRstToleranceR {
        Gpio174wrPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO175 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio175wr_privilege_rst_tolerance(&self) -> Gpio175wrPrivilegeRstToleranceR {
        Gpio175wrPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO176 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio176wr_privilege_rst_tolerance(&self) -> Gpio176wrPrivilegeRstToleranceR {
        Gpio176wrPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO177 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio177wr_privilege_rst_tolerance(&self) -> Gpio177wrPrivilegeRstToleranceR {
        Gpio177wrPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO178 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio178wr_privilege_rst_tolerance(&self) -> Gpio178wrPrivilegeRstToleranceR {
        Gpio178wrPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO179 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio179wr_privilege_rst_tolerance(&self) -> Gpio179wrPrivilegeRstToleranceR {
        Gpio179wrPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO180 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio180wr_privilege_rst_tolerance(&self) -> Gpio180wrPrivilegeRstToleranceR {
        Gpio180wrPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO181 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio181wr_privilege_rst_tolerance(&self) -> Gpio181wrPrivilegeRstToleranceR {
        Gpio181wrPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO182 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio182wr_privilege_rst_tolerance(&self) -> Gpio182wrPrivilegeRstToleranceR {
        Gpio182wrPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO183 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio183wr_privilege_rst_tolerance(&self) -> Gpio183wrPrivilegeRstToleranceR {
        Gpio183wrPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO184 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio184wr_privilege_rst_tolerance(&self) -> Gpio184wrPrivilegeRstToleranceR {
        Gpio184wrPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO185 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio185wr_privilege_rst_tolerance(&self) -> Gpio185wrPrivilegeRstToleranceR {
        Gpio185wrPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO186 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio186wr_privilege_rst_tolerance(&self) -> Gpio186wrPrivilegeRstToleranceR {
        Gpio186wrPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO187 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio187wr_privilege_rst_tolerance(&self) -> Gpio187wrPrivilegeRstToleranceR {
        Gpio187wrPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO188 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio188wr_privilege_rst_tolerance(&self) -> Gpio188wrPrivilegeRstToleranceR {
        Gpio188wrPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO189 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio189wr_privilege_rst_tolerance(&self) -> Gpio189wrPrivilegeRstToleranceR {
        Gpio189wrPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO190 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio190wr_privilege_rst_tolerance(&self) -> Gpio190wrPrivilegeRstToleranceR {
        Gpio190wrPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO191 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio191wr_privilege_rst_tolerance(&self) -> Gpio191wrPrivilegeRstToleranceR {
        Gpio191wrPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO160 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio160wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio160wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio160wrPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO161 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio161wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio161wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio161wrPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO162 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio162wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio162wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio162wrPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO163 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio163wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio163wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio163wrPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO164 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio164wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio164wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio164wrPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO165 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio165wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio165wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio165wrPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO166 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio166wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio166wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio166wrPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO167 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio167wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio167wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio167wrPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO168 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio168wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio168wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio168wrPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO169 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio169wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio169wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio169wrPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO170 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio170wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio170wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio170wrPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO171 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio171wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio171wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio171wrPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO172 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio172wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio172wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio172wrPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO173 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio173wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio173wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio173wrPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO174 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio174wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio174wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio174wrPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO175 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio175wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio175wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio175wrPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO176 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio176wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio176wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio176wrPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO177 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio177wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio177wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio177wrPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO178 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio178wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio178wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio178wrPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO179 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio179wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio179wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio179wrPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO180 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio180wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio180wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio180wrPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO181 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio181wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio181wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio181wrPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO182 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio182wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio182wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio182wrPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO183 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio183wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio183wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio183wrPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO184 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio184wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio184wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio184wrPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO185 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio185wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio185wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio185wrPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO186 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio186wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio186wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio186wrPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO187 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio187wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio187wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio187wrPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO188 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio188wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio188wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio188wrPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO189 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio189wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio189wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio189wrPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO190 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio190wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio190wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio190wrPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO191 Write Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio191wr_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio191wrPrivilegeRstToleranceW<Gpiob24Spec> {
        Gpio191wrPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Write Privilege Reset Tolerance Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiob24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiob24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpiob24Spec;
impl crate::RegisterSpec for Gpiob24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob24::R`](R) reader structure"]
impl crate::Readable for Gpiob24Spec {}
#[doc = "`write(|w| ..)` method takes [`gpiob24::W`](W) writer structure"]
impl crate::Writable for Gpiob24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOB24 to value 0"]
impl crate::Resettable for Gpiob24Spec {}
