#[doc = "Register `GPIO858` reader"]
pub type R = crate::R<Gpio858Spec>;
#[doc = "Register `GPIO858` writer"]
pub type W = crate::W<Gpio858Spec>;
#[doc = "Field `GPIO072WrPrivilegeOfMaster` reader - GPIO072 Write Privilege of Master"]
pub type Gpio072wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO072WrPrivilegeOfMaster` writer - GPIO072 Write Privilege of Master"]
pub type Gpio072wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO073WrPrivilegeOfMaster` reader - GPIO073 Write Privilege of Master"]
pub type Gpio073wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO073WrPrivilegeOfMaster` writer - GPIO073 Write Privilege of Master"]
pub type Gpio073wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO074WrPrivilegeOfMaster` reader - GPIO074 Write Privilege of Master"]
pub type Gpio074wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO074WrPrivilegeOfMaster` writer - GPIO074 Write Privilege of Master"]
pub type Gpio074wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO075WrPrivilegeOfMaster` reader - GPIO075 Write Privilege of Master"]
pub type Gpio075wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO075WrPrivilegeOfMaster` writer - GPIO075 Write Privilege of Master"]
pub type Gpio075wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO072 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio072wr_privilege_of_master(&self) -> Gpio072wrPrivilegeOfMasterR {
        Gpio072wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO073 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio073wr_privilege_of_master(&self) -> Gpio073wrPrivilegeOfMasterR {
        Gpio073wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO074 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio074wr_privilege_of_master(&self) -> Gpio074wrPrivilegeOfMasterR {
        Gpio074wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO075 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio075wr_privilege_of_master(&self) -> Gpio075wrPrivilegeOfMasterR {
        Gpio075wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO072 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio072wr_privilege_of_master(&mut self) -> Gpio072wrPrivilegeOfMasterW<Gpio858Spec> {
        Gpio072wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO073 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio073wr_privilege_of_master(&mut self) -> Gpio073wrPrivilegeOfMasterW<Gpio858Spec> {
        Gpio073wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO074 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio074wr_privilege_of_master(&mut self) -> Gpio074wrPrivilegeOfMasterW<Gpio858Spec> {
        Gpio074wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO075 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio075wr_privilege_of_master(&mut self) -> Gpio075wrPrivilegeOfMasterW<Gpio858Spec> {
        Gpio075wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio858::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio858::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio858Spec;
impl crate::RegisterSpec for Gpio858Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio858::R`](R) reader structure"]
impl crate::Readable for Gpio858Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio858::W`](W) writer structure"]
impl crate::Writable for Gpio858Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO858 to value 0xffff_ffff"]
impl crate::Resettable for Gpio858Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
