#[doc = "Register `GPIO964` reader"]
pub type R = crate::R<Gpio964Spec>;
#[doc = "Register `GPIO964` writer"]
pub type W = crate::W<Gpio964Spec>;
#[doc = "Field `GPIO084ReadPrivilegeOfMaster` reader - GPIO084 Read Privilege of Master"]
pub type Gpio084readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO084ReadPrivilegeOfMaster` writer - GPIO084 Read Privilege of Master"]
pub type Gpio084readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO085ReadPrivilegeOfMaster` reader - GPIO085 Read Privilege of Master"]
pub type Gpio085readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO085ReadPrivilegeOfMaster` writer - GPIO085 Read Privilege of Master"]
pub type Gpio085readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO086ReadPrivilegeOfMaster` reader - GPIO086 Read Privilege of Master"]
pub type Gpio086readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO086ReadPrivilegeOfMaster` writer - GPIO086 Read Privilege of Master"]
pub type Gpio086readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO087ReadPrivilegeOfMaster` reader - GPIO087 Read Privilege of Master"]
pub type Gpio087readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO087ReadPrivilegeOfMaster` writer - GPIO087 Read Privilege of Master"]
pub type Gpio087readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO084 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio084read_privilege_of_master(&self) -> Gpio084readPrivilegeOfMasterR {
        Gpio084readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO085 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio085read_privilege_of_master(&self) -> Gpio085readPrivilegeOfMasterR {
        Gpio085readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO086 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio086read_privilege_of_master(&self) -> Gpio086readPrivilegeOfMasterR {
        Gpio086readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO087 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio087read_privilege_of_master(&self) -> Gpio087readPrivilegeOfMasterR {
        Gpio087readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO084 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio084read_privilege_of_master(
        &mut self,
    ) -> Gpio084readPrivilegeOfMasterW<Gpio964Spec> {
        Gpio084readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO085 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio085read_privilege_of_master(
        &mut self,
    ) -> Gpio085readPrivilegeOfMasterW<Gpio964Spec> {
        Gpio085readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO086 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio086read_privilege_of_master(
        &mut self,
    ) -> Gpio086readPrivilegeOfMasterW<Gpio964Spec> {
        Gpio086readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO087 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio087read_privilege_of_master(
        &mut self,
    ) -> Gpio087readPrivilegeOfMasterW<Gpio964Spec> {
        Gpio087readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#21\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio964::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio964::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio964Spec;
impl crate::RegisterSpec for Gpio964Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio964::R`](R) reader structure"]
impl crate::Readable for Gpio964Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio964::W`](W) writer structure"]
impl crate::Writable for Gpio964Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO964 to value 0xffff_ffff"]
impl crate::Resettable for Gpio964Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
