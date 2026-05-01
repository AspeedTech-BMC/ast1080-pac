#[doc = "Register `GPIO92C` reader"]
pub type R = crate::R<Gpio92cSpec>;
#[doc = "Register `GPIO92C` writer"]
pub type W = crate::W<Gpio92cSpec>;
#[doc = "Field `GPIO028ReadPrivilegeOfMaster` reader - GPIO028 Read Privilege of Master"]
pub type Gpio028readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO028ReadPrivilegeOfMaster` writer - GPIO028 Read Privilege of Master"]
pub type Gpio028readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO029ReadPrivilegeOfMaster` reader - GPIO029 Read Privilege of Master"]
pub type Gpio029readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO029ReadPrivilegeOfMaster` writer - GPIO029 Read Privilege of Master"]
pub type Gpio029readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO030ReadPrivilegeOfMaster` reader - GPIO030 Read Privilege of Master"]
pub type Gpio030readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO030ReadPrivilegeOfMaster` writer - GPIO030 Read Privilege of Master"]
pub type Gpio030readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO031ReadPrivilegeOfMaster` reader - GPIO031 Read Privilege of Master"]
pub type Gpio031readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO031ReadPrivilegeOfMaster` writer - GPIO031 Read Privilege of Master"]
pub type Gpio031readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO028 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio028read_privilege_of_master(&self) -> Gpio028readPrivilegeOfMasterR {
        Gpio028readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO029 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio029read_privilege_of_master(&self) -> Gpio029readPrivilegeOfMasterR {
        Gpio029readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO030 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio030read_privilege_of_master(&self) -> Gpio030readPrivilegeOfMasterR {
        Gpio030readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO031 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio031read_privilege_of_master(&self) -> Gpio031readPrivilegeOfMasterR {
        Gpio031readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO028 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio028read_privilege_of_master(
        &mut self,
    ) -> Gpio028readPrivilegeOfMasterW<Gpio92cSpec> {
        Gpio028readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO029 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio029read_privilege_of_master(
        &mut self,
    ) -> Gpio029readPrivilegeOfMasterW<Gpio92cSpec> {
        Gpio029readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO030 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio030read_privilege_of_master(
        &mut self,
    ) -> Gpio030readPrivilegeOfMasterW<Gpio92cSpec> {
        Gpio030readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO031 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio031read_privilege_of_master(
        &mut self,
    ) -> Gpio031readPrivilegeOfMasterW<Gpio92cSpec> {
        Gpio031readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio92c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio92c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio92cSpec;
impl crate::RegisterSpec for Gpio92cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio92c::R`](R) reader structure"]
impl crate::Readable for Gpio92cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio92c::W`](W) writer structure"]
impl crate::Writable for Gpio92cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO92C to value 0xffff_ffff"]
impl crate::Resettable for Gpio92cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
