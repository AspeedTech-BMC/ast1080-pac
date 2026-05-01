#[doc = "Register `GPIO91C` reader"]
pub type R = crate::R<Gpio91cSpec>;
#[doc = "Register `GPIO91C` writer"]
pub type W = crate::W<Gpio91cSpec>;
#[doc = "Field `GPIO012ReadPrivilegeOfMaster` reader - GPIO012 Read Privilege of Master"]
pub type Gpio012readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO012ReadPrivilegeOfMaster` writer - GPIO012 Read Privilege of Master"]
pub type Gpio012readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO013ReadPrivilegeOfMaster` reader - GPIO013 Read Privilege of Master"]
pub type Gpio013readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO013ReadPrivilegeOfMaster` writer - GPIO013 Read Privilege of Master"]
pub type Gpio013readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO014ReadPrivilegeOfMaster` reader - GPIO014 Read Privilege of Master"]
pub type Gpio014readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO014ReadPrivilegeOfMaster` writer - GPIO014 Read Privilege of Master"]
pub type Gpio014readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO015ReadPrivilegeOfMaster` reader - GPIO015 Read Privilege of Master"]
pub type Gpio015readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO015ReadPrivilegeOfMaster` writer - GPIO015 Read Privilege of Master"]
pub type Gpio015readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO012 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio012read_privilege_of_master(&self) -> Gpio012readPrivilegeOfMasterR {
        Gpio012readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO013 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio013read_privilege_of_master(&self) -> Gpio013readPrivilegeOfMasterR {
        Gpio013readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO014 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio014read_privilege_of_master(&self) -> Gpio014readPrivilegeOfMasterR {
        Gpio014readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO015 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio015read_privilege_of_master(&self) -> Gpio015readPrivilegeOfMasterR {
        Gpio015readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO012 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio012read_privilege_of_master(
        &mut self,
    ) -> Gpio012readPrivilegeOfMasterW<Gpio91cSpec> {
        Gpio012readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO013 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio013read_privilege_of_master(
        &mut self,
    ) -> Gpio013readPrivilegeOfMasterW<Gpio91cSpec> {
        Gpio013readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO014 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio014read_privilege_of_master(
        &mut self,
    ) -> Gpio014readPrivilegeOfMasterW<Gpio91cSpec> {
        Gpio014readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO015 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio015read_privilege_of_master(
        &mut self,
    ) -> Gpio015readPrivilegeOfMasterW<Gpio91cSpec> {
        Gpio015readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio91c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio91c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio91cSpec;
impl crate::RegisterSpec for Gpio91cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio91c::R`](R) reader structure"]
impl crate::Readable for Gpio91cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio91c::W`](W) writer structure"]
impl crate::Writable for Gpio91cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO91C to value 0xffff_ffff"]
impl crate::Resettable for Gpio91cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
