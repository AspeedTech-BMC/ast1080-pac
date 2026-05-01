#[doc = "Register `GPIO848` reader"]
pub type R = crate::R<Gpio848Spec>;
#[doc = "Register `GPIO848` writer"]
pub type W = crate::W<Gpio848Spec>;
#[doc = "Field `GPIO056WrPrivilegeOfMaster` reader - GPIO056 Write Privilege of Master"]
pub type Gpio056wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO056WrPrivilegeOfMaster` writer - GPIO056 Write Privilege of Master"]
pub type Gpio056wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO057WrPrivilegeOfMaster` reader - GPIO057 Write Privilege of Master"]
pub type Gpio057wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO057WrPrivilegeOfMaster` writer - GPIO057 Write Privilege of Master"]
pub type Gpio057wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO058WrPrivilegeOfMaster` reader - GPIO058 Write Privilege of Master"]
pub type Gpio058wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO058WrPrivilegeOfMaster` writer - GPIO058 Write Privilege of Master"]
pub type Gpio058wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO059WrPrivilegeOfMaster` reader - GPIO059 Write Privilege of Master"]
pub type Gpio059wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO059WrPrivilegeOfMaster` writer - GPIO059 Write Privilege of Master"]
pub type Gpio059wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO056 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio056wr_privilege_of_master(&self) -> Gpio056wrPrivilegeOfMasterR {
        Gpio056wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO057 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio057wr_privilege_of_master(&self) -> Gpio057wrPrivilegeOfMasterR {
        Gpio057wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO058 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio058wr_privilege_of_master(&self) -> Gpio058wrPrivilegeOfMasterR {
        Gpio058wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO059 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio059wr_privilege_of_master(&self) -> Gpio059wrPrivilegeOfMasterR {
        Gpio059wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO056 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio056wr_privilege_of_master(&mut self) -> Gpio056wrPrivilegeOfMasterW<Gpio848Spec> {
        Gpio056wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO057 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio057wr_privilege_of_master(&mut self) -> Gpio057wrPrivilegeOfMasterW<Gpio848Spec> {
        Gpio057wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO058 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio058wr_privilege_of_master(&mut self) -> Gpio058wrPrivilegeOfMasterW<Gpio848Spec> {
        Gpio058wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO059 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio059wr_privilege_of_master(&mut self) -> Gpio059wrPrivilegeOfMasterW<Gpio848Spec> {
        Gpio059wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio848::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio848::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio848Spec;
impl crate::RegisterSpec for Gpio848Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio848::R`](R) reader structure"]
impl crate::Readable for Gpio848Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio848::W`](W) writer structure"]
impl crate::Writable for Gpio848Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO848 to value 0xffff_ffff"]
impl crate::Resettable for Gpio848Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
