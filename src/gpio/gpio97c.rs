#[doc = "Register `GPIO97C` reader"]
pub type R = crate::R<Gpio97cSpec>;
#[doc = "Register `GPIO97C` writer"]
pub type W = crate::W<Gpio97cSpec>;
#[doc = "Field `GPIO108ReadPrivilegeOfMaster` reader - GPIO108 Read Privilege of Master"]
pub type Gpio108readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO108ReadPrivilegeOfMaster` writer - GPIO108 Read Privilege of Master"]
pub type Gpio108readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO109ReadPrivilegeOfMaster` reader - GPIO109 Read Privilege of Master"]
pub type Gpio109readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO109ReadPrivilegeOfMaster` writer - GPIO109 Read Privilege of Master"]
pub type Gpio109readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO110ReadPrivilegeOfMaster` reader - GPIO110 Read Privilege of Master"]
pub type Gpio110readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO110ReadPrivilegeOfMaster` writer - GPIO110 Read Privilege of Master"]
pub type Gpio110readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO111ReadPrivilegeOfMaster` reader - GPIO111 Read Privilege of Master"]
pub type Gpio111readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO111ReadPrivilegeOfMaster` writer - GPIO111 Read Privilege of Master"]
pub type Gpio111readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO108 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio108read_privilege_of_master(&self) -> Gpio108readPrivilegeOfMasterR {
        Gpio108readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO109 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio109read_privilege_of_master(&self) -> Gpio109readPrivilegeOfMasterR {
        Gpio109readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO110 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio110read_privilege_of_master(&self) -> Gpio110readPrivilegeOfMasterR {
        Gpio110readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO111 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio111read_privilege_of_master(&self) -> Gpio111readPrivilegeOfMasterR {
        Gpio111readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO108 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio108read_privilege_of_master(
        &mut self,
    ) -> Gpio108readPrivilegeOfMasterW<Gpio97cSpec> {
        Gpio108readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO109 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio109read_privilege_of_master(
        &mut self,
    ) -> Gpio109readPrivilegeOfMasterW<Gpio97cSpec> {
        Gpio109readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO110 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio110read_privilege_of_master(
        &mut self,
    ) -> Gpio110readPrivilegeOfMasterW<Gpio97cSpec> {
        Gpio110readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO111 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio111read_privilege_of_master(
        &mut self,
    ) -> Gpio111readPrivilegeOfMasterW<Gpio97cSpec> {
        Gpio111readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#27\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio97c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio97c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio97cSpec;
impl crate::RegisterSpec for Gpio97cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio97c::R`](R) reader structure"]
impl crate::Readable for Gpio97cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio97c::W`](W) writer structure"]
impl crate::Writable for Gpio97cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO97C to value 0xffff_ffff"]
impl crate::Resettable for Gpio97cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
