#[doc = "Register `GPIO9C0` reader"]
pub type R = crate::R<Gpio9c0Spec>;
#[doc = "Register `GPIO9C0` writer"]
pub type W = crate::W<Gpio9c0Spec>;
#[doc = "Field `GPIO176ReadPrivilegeOfMaster` reader - GPIO176 Read Privilege of Master"]
pub type Gpio176readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO176ReadPrivilegeOfMaster` writer - GPIO176 Read Privilege of Master"]
pub type Gpio176readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO177ReadPrivilegeOfMaster` reader - GPIO177 Read Privilege of Master"]
pub type Gpio177readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO177ReadPrivilegeOfMaster` writer - GPIO177 Read Privilege of Master"]
pub type Gpio177readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO178ReadPrivilegeOfMaster` reader - GPIO178 Read Privilege of Master"]
pub type Gpio178readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO178ReadPrivilegeOfMaster` writer - GPIO178 Read Privilege of Master"]
pub type Gpio178readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO179ReadPrivilegeOfMaster` reader - GPIO179 Read Privilege of Master"]
pub type Gpio179readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO179ReadPrivilegeOfMaster` writer - GPIO179 Read Privilege of Master"]
pub type Gpio179readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO176 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio176read_privilege_of_master(&self) -> Gpio176readPrivilegeOfMasterR {
        Gpio176readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO177 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio177read_privilege_of_master(&self) -> Gpio177readPrivilegeOfMasterR {
        Gpio177readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO178 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio178read_privilege_of_master(&self) -> Gpio178readPrivilegeOfMasterR {
        Gpio178readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO179 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio179read_privilege_of_master(&self) -> Gpio179readPrivilegeOfMasterR {
        Gpio179readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO176 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio176read_privilege_of_master(
        &mut self,
    ) -> Gpio176readPrivilegeOfMasterW<Gpio9c0Spec> {
        Gpio176readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO177 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio177read_privilege_of_master(
        &mut self,
    ) -> Gpio177readPrivilegeOfMasterW<Gpio9c0Spec> {
        Gpio177readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO178 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio178read_privilege_of_master(
        &mut self,
    ) -> Gpio178readPrivilegeOfMasterW<Gpio9c0Spec> {
        Gpio178readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO179 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio179read_privilege_of_master(
        &mut self,
    ) -> Gpio179readPrivilegeOfMasterW<Gpio9c0Spec> {
        Gpio179readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9c0Spec;
impl crate::RegisterSpec for Gpio9c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9c0::R`](R) reader structure"]
impl crate::Readable for Gpio9c0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9c0::W`](W) writer structure"]
impl crate::Writable for Gpio9c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9C0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9c0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
