#[doc = "Register `SCU140` reader"]
pub type R = crate::R<Scu140Spec>;
#[doc = "Register `SCU140` writer"]
pub type W = crate::W<Scu140Spec>;
#[doc = "Field `SCUCPTRAZEROREQ` reader - SCU_CPTRA_ZERO_REQ"]
pub type ScucptrazeroreqR = crate::BitReader;
#[doc = "Field `SCUCPTRAZEROREQ` writer - SCU_CPTRA_ZERO_REQ"]
pub type ScucptrazeroreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCPTRAZEROTYPE` reader - SCU_CPTRA_ZERO_TYPE"]
pub type ScucptrazerotypeR = crate::BitReader;
#[doc = "Field `SCUCPTRAZEROTYPE` writer - SCU_CPTRA_ZERO_TYPE"]
pub type ScucptrazerotypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCPTRAFIPSZEROIZATIONPPDI` reader - SCU_CPTRA_FIPS_ZEROIZATION_PPD_I"]
pub type ScucptrafipszeroizationppdiR = crate::BitReader;
#[doc = "Field `SCUCPTRAFIPSZEROIZATIONPPDI` writer - SCU_CPTRA_FIPS_ZEROIZATION_PPD_I"]
pub type ScucptrafipszeroizationppdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCPTRACOREBOOTFSMBPI` reader - SCU_CPTRA_CORE_BOOTFSM_BP_I"]
pub type ScucptracorebootfsmbpiR = crate::BitReader;
#[doc = "Field `SCUCPTRACOREBOOTFSMBPI` writer - SCU_CPTRA_CORE_BOOTFSM_BP_I"]
pub type ScucptracorebootfsmbpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCPTRAALLOWRMAORSCRAPONPPDI` reader - SCU_CPTRA_ALLOW_RMA_OR_SCRAP_ON_PPD_I"]
pub type ScucptraallowrmaorscraponppdiR = crate::BitReader;
#[doc = "Field `SCUCPTRAALLOWRMAORSCRAPONPPDI` writer - SCU_CPTRA_ALLOW_RMA_OR_SCRAP_ON_PPD_I"]
pub type ScucptraallowrmaorscraponppdiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCPTRAESCLATESCRAPSTATE0I` reader - SCU_CPTRA_ESCLATE_SCRAP_STATE0_I"]
pub type Scucptraesclatescrapstate0iR = crate::BitReader;
#[doc = "Field `SCUCPTRAESCLATESCRAPSTATE0I` writer - SCU_CPTRA_ESCLATE_SCRAP_STATE0_I"]
pub type Scucptraesclatescrapstate0iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCPTRAESCLATESCRAPSTATE1I` reader - SCU_CPTRA_ESCLATE_SCRAP_STATE1_I"]
pub type Scucptraesclatescrapstate1iR = crate::BitReader;
#[doc = "Field `SCUCPTRAESCLATESCRAPSTATE1I` writer - SCU_CPTRA_ESCLATE_SCRAP_STATE1_I"]
pub type Scucptraesclatescrapstate1iW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCPTRAMCUNOROMCONFIGI` reader - SCU_CPTRA_MCU_NO_ROM_CONFIG_I"]
pub type ScucptramcunoromconfigiR = crate::BitReader;
#[doc = "Field `SCUCPTRAMCUNOROMCONFIGI` writer - SCU_CPTRA_MCU_NO_ROM_CONFIG_I"]
pub type ScucptramcunoromconfigiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_CPTRA_ZERO_REQ"]
    #[inline(always)]
    pub fn scucptrazeroreq(&self) -> ScucptrazeroreqR {
        ScucptrazeroreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_CPTRA_ZERO_TYPE"]
    #[inline(always)]
    pub fn scucptrazerotype(&self) -> ScucptrazerotypeR {
        ScucptrazerotypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_CPTRA_FIPS_ZEROIZATION_PPD_I"]
    #[inline(always)]
    pub fn scucptrafipszeroizationppdi(&self) -> ScucptrafipszeroizationppdiR {
        ScucptrafipszeroizationppdiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_CPTRA_CORE_BOOTFSM_BP_I"]
    #[inline(always)]
    pub fn scucptracorebootfsmbpi(&self) -> ScucptracorebootfsmbpiR {
        ScucptracorebootfsmbpiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_CPTRA_ALLOW_RMA_OR_SCRAP_ON_PPD_I"]
    #[inline(always)]
    pub fn scucptraallowrmaorscraponppdi(&self) -> ScucptraallowrmaorscraponppdiR {
        ScucptraallowrmaorscraponppdiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_CPTRA_ESCLATE_SCRAP_STATE0_I"]
    #[inline(always)]
    pub fn scucptraesclatescrapstate0i(&self) -> Scucptraesclatescrapstate0iR {
        Scucptraesclatescrapstate0iR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_CPTRA_ESCLATE_SCRAP_STATE1_I"]
    #[inline(always)]
    pub fn scucptraesclatescrapstate1i(&self) -> Scucptraesclatescrapstate1iR {
        Scucptraesclatescrapstate1iR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_CPTRA_MCU_NO_ROM_CONFIG_I"]
    #[inline(always)]
    pub fn scucptramcunoromconfigi(&self) -> ScucptramcunoromconfigiR {
        ScucptramcunoromconfigiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_CPTRA_ZERO_REQ"]
    #[inline(always)]
    pub fn scucptrazeroreq(&mut self) -> ScucptrazeroreqW<Scu140Spec> {
        ScucptrazeroreqW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_CPTRA_ZERO_TYPE"]
    #[inline(always)]
    pub fn scucptrazerotype(&mut self) -> ScucptrazerotypeW<Scu140Spec> {
        ScucptrazerotypeW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_CPTRA_FIPS_ZEROIZATION_PPD_I"]
    #[inline(always)]
    pub fn scucptrafipszeroizationppdi(&mut self) -> ScucptrafipszeroizationppdiW<Scu140Spec> {
        ScucptrafipszeroizationppdiW::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_CPTRA_CORE_BOOTFSM_BP_I"]
    #[inline(always)]
    pub fn scucptracorebootfsmbpi(&mut self) -> ScucptracorebootfsmbpiW<Scu140Spec> {
        ScucptracorebootfsmbpiW::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_CPTRA_ALLOW_RMA_OR_SCRAP_ON_PPD_I"]
    #[inline(always)]
    pub fn scucptraallowrmaorscraponppdi(&mut self) -> ScucptraallowrmaorscraponppdiW<Scu140Spec> {
        ScucptraallowrmaorscraponppdiW::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_CPTRA_ESCLATE_SCRAP_STATE0_I"]
    #[inline(always)]
    pub fn scucptraesclatescrapstate0i(&mut self) -> Scucptraesclatescrapstate0iW<Scu140Spec> {
        Scucptraesclatescrapstate0iW::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_CPTRA_ESCLATE_SCRAP_STATE1_I"]
    #[inline(always)]
    pub fn scucptraesclatescrapstate1i(&mut self) -> Scucptraesclatescrapstate1iW<Scu140Spec> {
        Scucptraesclatescrapstate1iW::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_CPTRA_MCU_NO_ROM_CONFIG_I"]
    #[inline(always)]
    pub fn scucptramcunoromconfigi(&mut self) -> ScucptramcunoromconfigiW<Scu140Spec> {
        ScucptramcunoromconfigiW::new(self, 7)
    }
}
#[doc = "Caliptra Config Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scu140::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu140::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu140Spec;
impl crate::RegisterSpec for Scu140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu140::R`](R) reader structure"]
impl crate::Readable for Scu140Spec {}
#[doc = "`write(|w| ..)` method takes [`scu140::W`](W) writer structure"]
impl crate::Writable for Scu140Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU140 to value 0"]
impl crate::Resettable for Scu140Spec {}
