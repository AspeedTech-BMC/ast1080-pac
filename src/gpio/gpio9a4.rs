#[doc = "Register `GPIO9A4` reader"]
pub type R = crate::R<Gpio9a4Spec>;
#[doc = "Register `GPIO9A4` writer"]
pub type W = crate::W<Gpio9a4Spec>;
#[doc = "Field `GPIO148ReadPrivilegeOfMaster` reader - GPIO148 Read Privilege of Master"]
pub type Gpio148readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO148ReadPrivilegeOfMaster` writer - GPIO148 Read Privilege of Master"]
pub type Gpio148readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO149ReadPrivilegeOfMaster` reader - GPIO149 Read Privilege of Master"]
pub type Gpio149readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO149ReadPrivilegeOfMaster` writer - GPIO149 Read Privilege of Master"]
pub type Gpio149readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO150ReadPrivilegeOfMaster` reader - GPIO150 Read Privilege of Master"]
pub type Gpio150readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO150ReadPrivilegeOfMaster` writer - GPIO150 Read Privilege of Master"]
pub type Gpio150readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO151ReadPrivilegeOfMaster` reader - GPIO151 Read Privilege of Master"]
pub type Gpio151readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO151ReadPrivilegeOfMaster` writer - GPIO151 Read Privilege of Master"]
pub type Gpio151readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO148 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio148read_privilege_of_master(&self) -> Gpio148readPrivilegeOfMasterR {
        Gpio148readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO149 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio149read_privilege_of_master(&self) -> Gpio149readPrivilegeOfMasterR {
        Gpio149readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO150 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio150read_privilege_of_master(&self) -> Gpio150readPrivilegeOfMasterR {
        Gpio150readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO151 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio151read_privilege_of_master(&self) -> Gpio151readPrivilegeOfMasterR {
        Gpio151readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO148 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio148read_privilege_of_master(
        &mut self,
    ) -> Gpio148readPrivilegeOfMasterW<Gpio9a4Spec> {
        Gpio148readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO149 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio149read_privilege_of_master(
        &mut self,
    ) -> Gpio149readPrivilegeOfMasterW<Gpio9a4Spec> {
        Gpio149readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO150 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio150read_privilege_of_master(
        &mut self,
    ) -> Gpio150readPrivilegeOfMasterW<Gpio9a4Spec> {
        Gpio150readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO151 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio151read_privilege_of_master(
        &mut self,
    ) -> Gpio151readPrivilegeOfMasterW<Gpio9a4Spec> {
        Gpio151readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#37\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9a4Spec;
impl crate::RegisterSpec for Gpio9a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9a4::R`](R) reader structure"]
impl crate::Readable for Gpio9a4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9a4::W`](W) writer structure"]
impl crate::Writable for Gpio9a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9A4 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9a4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
