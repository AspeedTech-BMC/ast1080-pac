#[doc = "Register `GPIO95C` reader"]
pub type R = crate::R<Gpio95cSpec>;
#[doc = "Register `GPIO95C` writer"]
pub type W = crate::W<Gpio95cSpec>;
#[doc = "Field `GPIO076ReadPrivilegeOfMaster` reader - GPIO076 Read Privilege of Master"]
pub type Gpio076readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO076ReadPrivilegeOfMaster` writer - GPIO076 Read Privilege of Master"]
pub type Gpio076readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO077ReadPrivilegeOfMaster` reader - GPIO077 Read Privilege of Master"]
pub type Gpio077readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO077ReadPrivilegeOfMaster` writer - GPIO077 Read Privilege of Master"]
pub type Gpio077readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO078ReadPrivilegeOfMaster` reader - GPIO078 Read Privilege of Master"]
pub type Gpio078readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO078ReadPrivilegeOfMaster` writer - GPIO078 Read Privilege of Master"]
pub type Gpio078readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO079ReadPrivilegeOfMaster` reader - GPIO079 Read Privilege of Master"]
pub type Gpio079readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO079ReadPrivilegeOfMaster` writer - GPIO079 Read Privilege of Master"]
pub type Gpio079readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO076 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio076read_privilege_of_master(&self) -> Gpio076readPrivilegeOfMasterR {
        Gpio076readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO077 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio077read_privilege_of_master(&self) -> Gpio077readPrivilegeOfMasterR {
        Gpio077readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO078 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio078read_privilege_of_master(&self) -> Gpio078readPrivilegeOfMasterR {
        Gpio078readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO079 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio079read_privilege_of_master(&self) -> Gpio079readPrivilegeOfMasterR {
        Gpio079readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO076 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio076read_privilege_of_master(
        &mut self,
    ) -> Gpio076readPrivilegeOfMasterW<Gpio95cSpec> {
        Gpio076readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO077 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio077read_privilege_of_master(
        &mut self,
    ) -> Gpio077readPrivilegeOfMasterW<Gpio95cSpec> {
        Gpio077readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO078 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio078read_privilege_of_master(
        &mut self,
    ) -> Gpio078readPrivilegeOfMasterW<Gpio95cSpec> {
        Gpio078readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO079 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio079read_privilege_of_master(
        &mut self,
    ) -> Gpio079readPrivilegeOfMasterW<Gpio95cSpec> {
        Gpio079readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio95c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio95c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio95cSpec;
impl crate::RegisterSpec for Gpio95cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio95c::R`](R) reader structure"]
impl crate::Readable for Gpio95cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio95c::W`](W) writer structure"]
impl crate::Writable for Gpio95cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO95C to value 0xffff_ffff"]
impl crate::Resettable for Gpio95cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
