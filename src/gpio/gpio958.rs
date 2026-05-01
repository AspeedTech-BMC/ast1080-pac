#[doc = "Register `GPIO958` reader"]
pub type R = crate::R<Gpio958Spec>;
#[doc = "Register `GPIO958` writer"]
pub type W = crate::W<Gpio958Spec>;
#[doc = "Field `GPIO072ReadPrivilegeOfMaster` reader - GPIO072 Read Privilege of Master"]
pub type Gpio072readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO072ReadPrivilegeOfMaster` writer - GPIO072 Read Privilege of Master"]
pub type Gpio072readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO073ReadPrivilegeOfMaster` reader - GPIO073 Read Privilege of Master"]
pub type Gpio073readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO073ReadPrivilegeOfMaster` writer - GPIO073 Read Privilege of Master"]
pub type Gpio073readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO074ReadPrivilegeOfMaster` reader - GPIO074 Read Privilege of Master"]
pub type Gpio074readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO074ReadPrivilegeOfMaster` writer - GPIO074 Read Privilege of Master"]
pub type Gpio074readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO075ReadPrivilegeOfMaster` reader - GPIO075 Read Privilege of Master"]
pub type Gpio075readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO075ReadPrivilegeOfMaster` writer - GPIO075 Read Privilege of Master"]
pub type Gpio075readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO072 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio072read_privilege_of_master(&self) -> Gpio072readPrivilegeOfMasterR {
        Gpio072readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO073 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio073read_privilege_of_master(&self) -> Gpio073readPrivilegeOfMasterR {
        Gpio073readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO074 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio074read_privilege_of_master(&self) -> Gpio074readPrivilegeOfMasterR {
        Gpio074readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO075 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio075read_privilege_of_master(&self) -> Gpio075readPrivilegeOfMasterR {
        Gpio075readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO072 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio072read_privilege_of_master(
        &mut self,
    ) -> Gpio072readPrivilegeOfMasterW<Gpio958Spec> {
        Gpio072readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO073 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio073read_privilege_of_master(
        &mut self,
    ) -> Gpio073readPrivilegeOfMasterW<Gpio958Spec> {
        Gpio073readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO074 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio074read_privilege_of_master(
        &mut self,
    ) -> Gpio074readPrivilegeOfMasterW<Gpio958Spec> {
        Gpio074readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO075 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio075read_privilege_of_master(
        &mut self,
    ) -> Gpio075readPrivilegeOfMasterW<Gpio958Spec> {
        Gpio075readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#18\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio958::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio958::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio958Spec;
impl crate::RegisterSpec for Gpio958Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio958::R`](R) reader structure"]
impl crate::Readable for Gpio958Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio958::W`](W) writer structure"]
impl crate::Writable for Gpio958Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO958 to value 0xffff_ffff"]
impl crate::Resettable for Gpio958Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
