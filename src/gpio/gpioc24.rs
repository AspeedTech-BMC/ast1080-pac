#[doc = "Register `GPIOC24` reader"]
pub type R = crate::R<Gpioc24Spec>;
#[doc = "Register `GPIOC24` writer"]
pub type W = crate::W<Gpioc24Spec>;
#[doc = "GPIO160 Read Privilege Reset Tolerance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gpio160readPrivilegeRstTolerance {
    #[doc = "0: Read Privilege of GPIO160 is reset by WDT."]
    ReadPrivilegeOfGpio160IsResetByWdt = 0,
    #[doc = "1: Read Privilege of GPIO160 is NOT reset by WDT."]
    ReadPrivilegeOfGpio160IsNotResetByWdt = 1,
}
impl From<Gpio160readPrivilegeRstTolerance> for bool {
    #[inline(always)]
    fn from(variant: Gpio160readPrivilegeRstTolerance) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO160ReadPrivilegeRstTolerance` reader - GPIO160 Read Privilege Reset Tolerance"]
pub type Gpio160readPrivilegeRstToleranceR = crate::BitReader<Gpio160readPrivilegeRstTolerance>;
impl Gpio160readPrivilegeRstToleranceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio160readPrivilegeRstTolerance {
        match self.bits {
            false => Gpio160readPrivilegeRstTolerance::ReadPrivilegeOfGpio160IsResetByWdt,
            true => Gpio160readPrivilegeRstTolerance::ReadPrivilegeOfGpio160IsNotResetByWdt,
        }
    }
    #[doc = "Read Privilege of GPIO160 is reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio160_is_reset_by_wdt(&self) -> bool {
        *self == Gpio160readPrivilegeRstTolerance::ReadPrivilegeOfGpio160IsResetByWdt
    }
    #[doc = "Read Privilege of GPIO160 is NOT reset by WDT."]
    #[inline(always)]
    pub fn is_read_privilege_of_gpio160_is_not_reset_by_wdt(&self) -> bool {
        *self == Gpio160readPrivilegeRstTolerance::ReadPrivilegeOfGpio160IsNotResetByWdt
    }
}
#[doc = "Field `GPIO160ReadPrivilegeRstTolerance` writer - GPIO160 Read Privilege Reset Tolerance"]
pub type Gpio160readPrivilegeRstToleranceW<'a, REG> =
    crate::BitWriter<'a, REG, Gpio160readPrivilegeRstTolerance>;
impl<'a, REG> Gpio160readPrivilegeRstToleranceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read Privilege of GPIO160 is reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio160_is_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio160readPrivilegeRstTolerance::ReadPrivilegeOfGpio160IsResetByWdt)
    }
    #[doc = "Read Privilege of GPIO160 is NOT reset by WDT."]
    #[inline(always)]
    pub fn read_privilege_of_gpio160_is_not_reset_by_wdt(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio160readPrivilegeRstTolerance::ReadPrivilegeOfGpio160IsNotResetByWdt)
    }
}
#[doc = "Field `GPIO161ReadPrivilegeRstTolerance` reader - GPIO161 Read Privilege Reset Tolerance"]
pub type Gpio161readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO161ReadPrivilegeRstTolerance` writer - GPIO161 Read Privilege Reset Tolerance"]
pub type Gpio161readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO162ReadPrivilegeRstTolerance` reader - GPIO162 Read Privilege Reset Tolerance"]
pub type Gpio162readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO162ReadPrivilegeRstTolerance` writer - GPIO162 Read Privilege Reset Tolerance"]
pub type Gpio162readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO163ReadPrivilegeRstTolerance` reader - GPIO163 Read Privilege Reset Tolerance"]
pub type Gpio163readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO163ReadPrivilegeRstTolerance` writer - GPIO163 Read Privilege Reset Tolerance"]
pub type Gpio163readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO164ReadPrivilegeRstTolerance` reader - GPIO164 Read Privilege Reset Tolerance"]
pub type Gpio164readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO164ReadPrivilegeRstTolerance` writer - GPIO164 Read Privilege Reset Tolerance"]
pub type Gpio164readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO165ReadPrivilegeRstTolerance` reader - GPIO165 Read Privilege Reset Tolerance"]
pub type Gpio165readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO165ReadPrivilegeRstTolerance` writer - GPIO165 Read Privilege Reset Tolerance"]
pub type Gpio165readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO166ReadPrivilegeRstTolerance` reader - GPIO166 Read Privilege Reset Tolerance"]
pub type Gpio166readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO166ReadPrivilegeRstTolerance` writer - GPIO166 Read Privilege Reset Tolerance"]
pub type Gpio166readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO167ReadPrivilegeRstTolerance` reader - GPIO167 Read Privilege Reset Tolerance"]
pub type Gpio167readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO167ReadPrivilegeRstTolerance` writer - GPIO167 Read Privilege Reset Tolerance"]
pub type Gpio167readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO168ReadPrivilegeRstTolerance` reader - GPIO168 Read Privilege Reset Tolerance"]
pub type Gpio168readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO168ReadPrivilegeRstTolerance` writer - GPIO168 Read Privilege Reset Tolerance"]
pub type Gpio168readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO169ReadPrivilegeRstTolerance` reader - GPIO169 Read Privilege Reset Tolerance"]
pub type Gpio169readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO169ReadPrivilegeRstTolerance` writer - GPIO169 Read Privilege Reset Tolerance"]
pub type Gpio169readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO170ReadPrivilegeRstTolerance` reader - GPIO170 Read Privilege Reset Tolerance"]
pub type Gpio170readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO170ReadPrivilegeRstTolerance` writer - GPIO170 Read Privilege Reset Tolerance"]
pub type Gpio170readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO171ReadPrivilegeRstTolerance` reader - GPIO171 Read Privilege Reset Tolerance"]
pub type Gpio171readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO171ReadPrivilegeRstTolerance` writer - GPIO171 Read Privilege Reset Tolerance"]
pub type Gpio171readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO172ReadPrivilegeRstTolerance` reader - GPIO172 Read Privilege Reset Tolerance"]
pub type Gpio172readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO172ReadPrivilegeRstTolerance` writer - GPIO172 Read Privilege Reset Tolerance"]
pub type Gpio172readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO173ReadPrivilegeRstTolerance` reader - GPIO173 Read Privilege Reset Tolerance"]
pub type Gpio173readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO173ReadPrivilegeRstTolerance` writer - GPIO173 Read Privilege Reset Tolerance"]
pub type Gpio173readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO174ReadPrivilegeRstTolerance` reader - GPIO174 Read Privilege Reset Tolerance"]
pub type Gpio174readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO174ReadPrivilegeRstTolerance` writer - GPIO174 Read Privilege Reset Tolerance"]
pub type Gpio174readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO175ReadPrivilegeRstTolerance` reader - GPIO175 Read Privilege Reset Tolerance"]
pub type Gpio175readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO175ReadPrivilegeRstTolerance` writer - GPIO175 Read Privilege Reset Tolerance"]
pub type Gpio175readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO176ReadPrivilegeRstTolerance` reader - GPIO176 Read Privilege Reset Tolerance"]
pub type Gpio176readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO176ReadPrivilegeRstTolerance` writer - GPIO176 Read Privilege Reset Tolerance"]
pub type Gpio176readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO177ReadPrivilegeRstTolerance` reader - GPIO177 Read Privilege Reset Tolerance"]
pub type Gpio177readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO177ReadPrivilegeRstTolerance` writer - GPIO177 Read Privilege Reset Tolerance"]
pub type Gpio177readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO178ReadPrivilegeRstTolerance` reader - GPIO178 Read Privilege Reset Tolerance"]
pub type Gpio178readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO178ReadPrivilegeRstTolerance` writer - GPIO178 Read Privilege Reset Tolerance"]
pub type Gpio178readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO179ReadPrivilegeRstTolerance` reader - GPIO179 Read Privilege Reset Tolerance"]
pub type Gpio179readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO179ReadPrivilegeRstTolerance` writer - GPIO179 Read Privilege Reset Tolerance"]
pub type Gpio179readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO180ReadPrivilegeRstTolerance` reader - GPIO180 Read Privilege Reset Tolerance"]
pub type Gpio180readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO180ReadPrivilegeRstTolerance` writer - GPIO180 Read Privilege Reset Tolerance"]
pub type Gpio180readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO181ReadPrivilegeRstTolerance` reader - GPIO181 Read Privilege Reset Tolerance"]
pub type Gpio181readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO181ReadPrivilegeRstTolerance` writer - GPIO181 Read Privilege Reset Tolerance"]
pub type Gpio181readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO182ReadPrivilegeRstTolerance` reader - GPIO182 Read Privilege Reset Tolerance"]
pub type Gpio182readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO182ReadPrivilegeRstTolerance` writer - GPIO182 Read Privilege Reset Tolerance"]
pub type Gpio182readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO183ReadPrivilegeRstTolerance` reader - GPIO183 Read Privilege Reset Tolerance"]
pub type Gpio183readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO183ReadPrivilegeRstTolerance` writer - GPIO183 Read Privilege Reset Tolerance"]
pub type Gpio183readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO184ReadPrivilegeRstTolerance` reader - GPIO184 Read Privilege Reset Tolerance"]
pub type Gpio184readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO184ReadPrivilegeRstTolerance` writer - GPIO184 Read Privilege Reset Tolerance"]
pub type Gpio184readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO185ReadPrivilegeRstTolerance` reader - GPIO185 Read Privilege Reset Tolerance"]
pub type Gpio185readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO185ReadPrivilegeRstTolerance` writer - GPIO185 Read Privilege Reset Tolerance"]
pub type Gpio185readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO186ReadPrivilegeRstTolerance` reader - GPIO186 Read Privilege Reset Tolerance"]
pub type Gpio186readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO186ReadPrivilegeRstTolerance` writer - GPIO186 Read Privilege Reset Tolerance"]
pub type Gpio186readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO187ReadPrivilegeRstTolerance` reader - GPIO187 Read Privilege Reset Tolerance"]
pub type Gpio187readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO187ReadPrivilegeRstTolerance` writer - GPIO187 Read Privilege Reset Tolerance"]
pub type Gpio187readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO188ReadPrivilegeRstTolerance` reader - GPIO188 Read Privilege Reset Tolerance"]
pub type Gpio188readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO188ReadPrivilegeRstTolerance` writer - GPIO188 Read Privilege Reset Tolerance"]
pub type Gpio188readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO189ReadPrivilegeRstTolerance` reader - GPIO189 Read Privilege Reset Tolerance"]
pub type Gpio189readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO189ReadPrivilegeRstTolerance` writer - GPIO189 Read Privilege Reset Tolerance"]
pub type Gpio189readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO190ReadPrivilegeRstTolerance` reader - GPIO190 Read Privilege Reset Tolerance"]
pub type Gpio190readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO190ReadPrivilegeRstTolerance` writer - GPIO190 Read Privilege Reset Tolerance"]
pub type Gpio190readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO191ReadPrivilegeRstTolerance` reader - GPIO191 Read Privilege Reset Tolerance"]
pub type Gpio191readPrivilegeRstToleranceR = crate::BitReader;
#[doc = "Field `GPIO191ReadPrivilegeRstTolerance` writer - GPIO191 Read Privilege Reset Tolerance"]
pub type Gpio191readPrivilegeRstToleranceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO160 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio160read_privilege_rst_tolerance(&self) -> Gpio160readPrivilegeRstToleranceR {
        Gpio160readPrivilegeRstToleranceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO161 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio161read_privilege_rst_tolerance(&self) -> Gpio161readPrivilegeRstToleranceR {
        Gpio161readPrivilegeRstToleranceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO162 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio162read_privilege_rst_tolerance(&self) -> Gpio162readPrivilegeRstToleranceR {
        Gpio162readPrivilegeRstToleranceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO163 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio163read_privilege_rst_tolerance(&self) -> Gpio163readPrivilegeRstToleranceR {
        Gpio163readPrivilegeRstToleranceR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO164 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio164read_privilege_rst_tolerance(&self) -> Gpio164readPrivilegeRstToleranceR {
        Gpio164readPrivilegeRstToleranceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO165 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio165read_privilege_rst_tolerance(&self) -> Gpio165readPrivilegeRstToleranceR {
        Gpio165readPrivilegeRstToleranceR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO166 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio166read_privilege_rst_tolerance(&self) -> Gpio166readPrivilegeRstToleranceR {
        Gpio166readPrivilegeRstToleranceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO167 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio167read_privilege_rst_tolerance(&self) -> Gpio167readPrivilegeRstToleranceR {
        Gpio167readPrivilegeRstToleranceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO168 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio168read_privilege_rst_tolerance(&self) -> Gpio168readPrivilegeRstToleranceR {
        Gpio168readPrivilegeRstToleranceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO169 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio169read_privilege_rst_tolerance(&self) -> Gpio169readPrivilegeRstToleranceR {
        Gpio169readPrivilegeRstToleranceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO170 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio170read_privilege_rst_tolerance(&self) -> Gpio170readPrivilegeRstToleranceR {
        Gpio170readPrivilegeRstToleranceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPIO171 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio171read_privilege_rst_tolerance(&self) -> Gpio171readPrivilegeRstToleranceR {
        Gpio171readPrivilegeRstToleranceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO172 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio172read_privilege_rst_tolerance(&self) -> Gpio172readPrivilegeRstToleranceR {
        Gpio172readPrivilegeRstToleranceR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GPIO173 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio173read_privilege_rst_tolerance(&self) -> Gpio173readPrivilegeRstToleranceR {
        Gpio173readPrivilegeRstToleranceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GPIO174 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio174read_privilege_rst_tolerance(&self) -> Gpio174readPrivilegeRstToleranceR {
        Gpio174readPrivilegeRstToleranceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GPIO175 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio175read_privilege_rst_tolerance(&self) -> Gpio175readPrivilegeRstToleranceR {
        Gpio175readPrivilegeRstToleranceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - GPIO176 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio176read_privilege_rst_tolerance(&self) -> Gpio176readPrivilegeRstToleranceR {
        Gpio176readPrivilegeRstToleranceR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - GPIO177 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio177read_privilege_rst_tolerance(&self) -> Gpio177readPrivilegeRstToleranceR {
        Gpio177readPrivilegeRstToleranceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - GPIO178 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio178read_privilege_rst_tolerance(&self) -> Gpio178readPrivilegeRstToleranceR {
        Gpio178readPrivilegeRstToleranceR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GPIO179 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio179read_privilege_rst_tolerance(&self) -> Gpio179readPrivilegeRstToleranceR {
        Gpio179readPrivilegeRstToleranceR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - GPIO180 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio180read_privilege_rst_tolerance(&self) -> Gpio180readPrivilegeRstToleranceR {
        Gpio180readPrivilegeRstToleranceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - GPIO181 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio181read_privilege_rst_tolerance(&self) -> Gpio181readPrivilegeRstToleranceR {
        Gpio181readPrivilegeRstToleranceR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - GPIO182 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio182read_privilege_rst_tolerance(&self) -> Gpio182readPrivilegeRstToleranceR {
        Gpio182readPrivilegeRstToleranceR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPIO183 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio183read_privilege_rst_tolerance(&self) -> Gpio183readPrivilegeRstToleranceR {
        Gpio183readPrivilegeRstToleranceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - GPIO184 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio184read_privilege_rst_tolerance(&self) -> Gpio184readPrivilegeRstToleranceR {
        Gpio184readPrivilegeRstToleranceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - GPIO185 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio185read_privilege_rst_tolerance(&self) -> Gpio185readPrivilegeRstToleranceR {
        Gpio185readPrivilegeRstToleranceR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - GPIO186 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio186read_privilege_rst_tolerance(&self) -> Gpio186readPrivilegeRstToleranceR {
        Gpio186readPrivilegeRstToleranceR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPIO187 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio187read_privilege_rst_tolerance(&self) -> Gpio187readPrivilegeRstToleranceR {
        Gpio187readPrivilegeRstToleranceR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GPIO188 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio188read_privilege_rst_tolerance(&self) -> Gpio188readPrivilegeRstToleranceR {
        Gpio188readPrivilegeRstToleranceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPIO189 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio189read_privilege_rst_tolerance(&self) -> Gpio189readPrivilegeRstToleranceR {
        Gpio189readPrivilegeRstToleranceR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GPIO190 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio190read_privilege_rst_tolerance(&self) -> Gpio190readPrivilegeRstToleranceR {
        Gpio190readPrivilegeRstToleranceR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GPIO191 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio191read_privilege_rst_tolerance(&self) -> Gpio191readPrivilegeRstToleranceR {
        Gpio191readPrivilegeRstToleranceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO160 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio160read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio160readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio160readPrivilegeRstToleranceW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO161 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio161read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio161readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio161readPrivilegeRstToleranceW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO162 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio162read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio162readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio162readPrivilegeRstToleranceW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO163 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio163read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio163readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio163readPrivilegeRstToleranceW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO164 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio164read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio164readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio164readPrivilegeRstToleranceW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO165 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio165read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio165readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio165readPrivilegeRstToleranceW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO166 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio166read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio166readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio166readPrivilegeRstToleranceW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO167 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio167read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio167readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio167readPrivilegeRstToleranceW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO168 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio168read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio168readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio168readPrivilegeRstToleranceW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO169 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio169read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio169readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio169readPrivilegeRstToleranceW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO170 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio170read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio170readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio170readPrivilegeRstToleranceW::new(self, 10)
    }
    #[doc = "Bit 11 - GPIO171 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio171read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio171readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio171readPrivilegeRstToleranceW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO172 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio172read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio172readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio172readPrivilegeRstToleranceW::new(self, 12)
    }
    #[doc = "Bit 13 - GPIO173 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio173read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio173readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio173readPrivilegeRstToleranceW::new(self, 13)
    }
    #[doc = "Bit 14 - GPIO174 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio174read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio174readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio174readPrivilegeRstToleranceW::new(self, 14)
    }
    #[doc = "Bit 15 - GPIO175 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio175read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio175readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio175readPrivilegeRstToleranceW::new(self, 15)
    }
    #[doc = "Bit 16 - GPIO176 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio176read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio176readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio176readPrivilegeRstToleranceW::new(self, 16)
    }
    #[doc = "Bit 17 - GPIO177 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio177read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio177readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio177readPrivilegeRstToleranceW::new(self, 17)
    }
    #[doc = "Bit 18 - GPIO178 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio178read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio178readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio178readPrivilegeRstToleranceW::new(self, 18)
    }
    #[doc = "Bit 19 - GPIO179 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio179read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio179readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio179readPrivilegeRstToleranceW::new(self, 19)
    }
    #[doc = "Bit 20 - GPIO180 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio180read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio180readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio180readPrivilegeRstToleranceW::new(self, 20)
    }
    #[doc = "Bit 21 - GPIO181 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio181read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio181readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio181readPrivilegeRstToleranceW::new(self, 21)
    }
    #[doc = "Bit 22 - GPIO182 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio182read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio182readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio182readPrivilegeRstToleranceW::new(self, 22)
    }
    #[doc = "Bit 23 - GPIO183 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio183read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio183readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio183readPrivilegeRstToleranceW::new(self, 23)
    }
    #[doc = "Bit 24 - GPIO184 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio184read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio184readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio184readPrivilegeRstToleranceW::new(self, 24)
    }
    #[doc = "Bit 25 - GPIO185 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio185read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio185readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio185readPrivilegeRstToleranceW::new(self, 25)
    }
    #[doc = "Bit 26 - GPIO186 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio186read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio186readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio186readPrivilegeRstToleranceW::new(self, 26)
    }
    #[doc = "Bit 27 - GPIO187 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio187read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio187readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio187readPrivilegeRstToleranceW::new(self, 27)
    }
    #[doc = "Bit 28 - GPIO188 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio188read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio188readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio188readPrivilegeRstToleranceW::new(self, 28)
    }
    #[doc = "Bit 29 - GPIO189 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio189read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio189readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio189readPrivilegeRstToleranceW::new(self, 29)
    }
    #[doc = "Bit 30 - GPIO190 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio190read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio190readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio190readPrivilegeRstToleranceW::new(self, 30)
    }
    #[doc = "Bit 31 - GPIO191 Read Privilege Reset Tolerance"]
    #[inline(always)]
    pub fn gpio191read_privilege_rst_tolerance(
        &mut self,
    ) -> Gpio191readPrivilegeRstToleranceW<Gpioc24Spec> {
        Gpio191readPrivilegeRstToleranceW::new(self, 31)
    }
}
#[doc = "Read Privilege Reset Tolerance Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpioc24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpioc24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpioc24Spec;
impl crate::RegisterSpec for Gpioc24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioc24::R`](R) reader structure"]
impl crate::Readable for Gpioc24Spec {}
#[doc = "`write(|w| ..)` method takes [`gpioc24::W`](W) writer structure"]
impl crate::Writable for Gpioc24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOC24 to value 0"]
impl crate::Resettable for Gpioc24Spec {}
